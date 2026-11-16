//! Twitch "live"-meldingen via de Helix API (app access token, client-credentials flow).

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context as _, bail};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::{debug, warn};

use crate::{
    app::App,
    config::TwitchConfig,
    db::{self, Kind},
    integrations::broadcast,
};

const TWITCH_PAARS: u32 = 0x9146FF;
/// Helix staat maximaal 100 logins per request toe.
const BATCH: usize = 100;

pub struct TwitchClient {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
    token: Mutex<Option<(String, Instant)>>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Deserialize)]
struct Page<T> {
    data: Vec<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
    pub display_name: String,
    pub profile_image_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stream {
    pub id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_name: String,
    pub title: String,
    pub thumbnail_url: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl TwitchClient {
    pub fn new(http: reqwest::Client, config: TwitchConfig) -> Self {
        Self {
            http,
            client_id: config.client_id,
            client_secret: config.client_secret,
            token: Mutex::new(None),
        }
    }

    async fn token(&self, force_refresh: bool) -> anyhow::Result<String> {
        let mut guard = self.token.lock().await;
        if let Some((token, expires)) = guard.as_ref()
            && !force_refresh
            && Instant::now() < *expires
        {
            return Ok(token.clone());
        }
        let resp: TokenResponse = self
            .http
            .post("https://id.twitch.tv/oauth2/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await?
            .error_for_status()
            .context("Twitch-token aanvragen mislukt (klopt TWITCH_CLIENT_ID/SECRET?)")?
            .json()
            .await?;
        // Een minuut marge zodat de token niet midden in een request verloopt.
        let expires = Instant::now() + Duration::from_secs(resp.expires_in.saturating_sub(60));
        *guard = Some((resp.access_token.clone(), expires));
        Ok(resp.access_token)
    }

    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> anyhow::Result<Vec<T>> {
        let url = format!("https://api.twitch.tv/helix/{path}");
        let mut refreshed = false;
        loop {
            let token = self.token(refreshed).await?;
            let resp = self
                .http
                .get(&url)
                .query(query)
                .header("Client-Id", &self.client_id)
                .bearer_auth(token)
                .send()
                .await?;
            if resp.status() == reqwest::StatusCode::UNAUTHORIZED && !refreshed {
                refreshed = true;
                continue;
            }
            let page: Page<T> = resp
                .error_for_status()
                .with_context(|| format!("Twitch API {path} gaf een fout"))?
                .json()
                .await?;
            return Ok(page.data);
        }
    }

    pub async fn user(&self, login: &str) -> anyhow::Result<Option<User>> {
        Ok(self
            .get("users", &[("login", login)])
            .await?
            .into_iter()
            .next())
    }

    pub async fn live_streams(&self, logins: &[&str]) -> anyhow::Result<Vec<Stream>> {
        let mut query: Vec<(&str, &str)> = logins.iter().map(|l| ("user_login", *l)).collect();
        query.push(("type", "live"));
        query.push(("first", "100"));
        self.get("streams", &query).await
    }
}

/// Maakt van invoer als `https://twitch.tv/Naam` of `@naam` een geldige Twitch-login.
pub fn normalize_login(input: &str) -> anyhow::Result<String> {
    let s = input.trim();
    let s = s
        .rsplit_once("twitch.tv/")
        .map(|(_, rest)| rest)
        .unwrap_or(s)
        .trim_start_matches('@');
    let login = s
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let valid = (3..=25).contains(&login.len())
        && login.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !valid {
        bail!("`{input}` is geen geldige Twitch-naam");
    }
    Ok(login)
}

pub async fn poll(app: Arc<App>) -> anyhow::Result<()> {
    let Some(twitch) = &app.twitch else {
        return Ok(());
    };
    let subs = db::list_by_kind(&app.db, Kind::Twitch).await?;
    if subs.is_empty() {
        return Ok(());
    }

    let mut logins: Vec<&str> = subs.iter().map(|s| s.target.as_str()).collect();
    logins.sort_unstable();
    logins.dedup();
    debug!(kanalen = logins.len(), "Twitch-ronde");

    let mut live = Vec::new();
    for chunk in logins.chunks(BATCH) {
        live.extend(twitch.live_streams(chunk).await?);
    }

    for sub in &subs {
        let Some(stream) = live
            .iter()
            .find(|s| s.user_login.eq_ignore_ascii_case(&sub.target))
        else {
            continue;
        };
        // Elke uitzending heeft een eigen stream-ID; die onthouden we zodat we één keer melden.
        // We wissen de toestand niet bij offline gaan, zodat een korte API-hapering of
        // herverbinding met dezelfde stream geen dubbele melding geeft.
        if sub.state.as_deref() == Some(stream.id.as_str()) {
            continue;
        }
        db::set_state(&app.db, sub.id, Some(&stream.id)).await?;
        if let Err(e) = broadcast(&app, sub, embed(stream)).await {
            warn!(sub = sub.id, "Twitch-melding mislukt: {e:#}");
        }
    }
    Ok(())
}

fn embed(stream: &Stream) -> serenity::CreateEmbed {
    let url = format!("https://www.twitch.tv/{}", stream.user_login);
    // Twitch cachet thumbnails agressief; de stream-ID als query maakt elke melding uniek.
    let thumbnail = format!(
        "{}?b={}",
        stream
            .thumbnail_url
            .replace("{width}", "1280")
            .replace("{height}", "720"),
        stream.id
    );
    let mut embed = serenity::CreateEmbed::new()
        .author(
            serenity::CreateEmbedAuthor::new(format!("🔴 {} is live!", stream.user_name)).url(&url),
        )
        .title(crate::util::truncate(&stream.title, 250))
        .url(&url)
        .image(thumbnail)
        .colour(TWITCH_PAARS)
        .timestamp(stream.started_at)
        .footer(serenity::CreateEmbedFooter::new("🟣 Broodkast · Twitch"));
    if !stream.game_name.is_empty() {
        embed = embed.field("Speelt", &stream.game_name, true);
    }
    embed
}

#[cfg(test)]
mod tests {
    use super::normalize_login;

    #[test]
    fn logins_normaliseren() {
        assert_eq!(
            normalize_login("https://www.twitch.tv/Shroud").unwrap(),
            "shroud"
        );
        assert_eq!(normalize_login("@Lirik").unwrap(), "lirik");
        assert_eq!(
            normalize_login("twitch.tv/some_one/videos").unwrap(),
            "some_one"
        );
        assert!(normalize_login("geen spaties").is_err());
        assert!(normalize_login("").is_err());
    }
}
