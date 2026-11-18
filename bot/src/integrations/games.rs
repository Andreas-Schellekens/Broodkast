//! Gratis games op Epic Games Store en Steam, via de gratis API van GamerPower.
//!
//! Toestand per abonnement: JSON-lijst met ID's van giveaways die al gemeld zijn. Alles wat er
//! niet in staat, is nieuw. Bij een nieuw abonnement staan de lopende giveaways er al in.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::Context as _;
use chrono::{NaiveDateTime, TimeZone, Utc};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use tracing::{debug, warn};

use crate::{
    app::App,
    db::{self, Kind, Subscription},
    integrations::{BROODKAST_KLEUR, broadcast},
    util::truncate,
};

const API_URL: &str = "https://www.gamerpower.com/api/filter";
/// Maximaal aantal meldingen per abonnement per ronde.
const MAX_PER_POLL: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum Platform {
    #[name = "Epic Games Store"]
    Epic,
    #[name = "Steam"]
    Steam,
    #[name = "Epic en Steam"]
    Alle,
}

impl Platform {
    /// Waarde in de kolom `target`.
    pub fn as_target(self) -> &'static str {
        match self {
            Platform::Epic => "epic",
            Platform::Steam => "steam",
            Platform::Alle => "alle",
        }
    }

    fn from_target(target: &str) -> Option<Self> {
        match target {
            "epic" => Some(Platform::Epic),
            "steam" => Some(Platform::Steam),
            "alle" => Some(Platform::Alle),
            _ => None,
        }
    }

    fn api_filter(self) -> &'static str {
        match self {
            Platform::Epic => "epic-games-store",
            Platform::Steam => "steam",
            Platform::Alle => "epic-games-store.steam",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Platform::Epic => "Gratis games op Epic",
            Platform::Steam => "Gratis games op Steam",
            Platform::Alle => "Gratis games op Epic en Steam",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Giveaway {
    id: u64,
    title: String,
    worth: String,
    image: String,
    description: String,
    open_giveaway_url: String,
    end_date: String,
    platforms: String,
    status: String,
}

/// Lopende giveaways voor een platform (alleen volledige games, geen DLC of loot).
pub async fn fetch(http: &reqwest::Client, platform: Platform) -> anyhow::Result<Vec<Giveaway>> {
    let resp = http
        .get(API_URL)
        .query(&[("platform", platform.api_filter()), ("type", "game")])
        .send()
        .await
        .context("GamerPower niet bereikbaar")?
        .error_for_status()
        .context("GamerPower gaf een fout")?;
    // Zonder lopende giveaways antwoordt GamerPower met een object in plaats van een lijst.
    let body: serde_json::Value = resp.json().await.context("GamerPower gaf geen JSON")?;
    if !body.is_array() {
        return Ok(Vec::new());
    }
    let giveaways: Vec<Giveaway> =
        serde_json::from_value(body).context("GamerPower gaf onverwachte data")?;
    Ok(giveaways
        .into_iter()
        .filter(|g| g.status.eq_ignore_ascii_case("active"))
        .collect())
}

pub fn initial_state(giveaways: &[Giveaway]) -> String {
    serde_json::to_string(&giveaways.iter().map(|g| g.id).collect::<Vec<_>>())
        .expect("getallen serialiseren faalt niet")
}

fn parse_seen(state: Option<&str>) -> Option<HashSet<u64>> {
    serde_json::from_str::<Vec<u64>>(state?)
        .ok()
        .map(HashSet::from_iter)
}

/// Nieuwe giveaways, oudste eerst (GamerPower-ID's lopen op).
fn new_giveaways<'a>(giveaways: &'a [Giveaway], seen: &HashSet<u64>) -> Vec<&'a Giveaway> {
    let mut fresh: Vec<&Giveaway> = giveaways.iter().filter(|g| !seen.contains(&g.id)).collect();
    fresh.sort_by_key(|g| g.id);
    fresh.truncate(MAX_PER_POLL);
    fresh
}

pub async fn poll(app: Arc<App>) -> anyhow::Result<()> {
    let subs = db::list_by_kind(&app.db, Kind::Games).await?;
    let mut by_platform: HashMap<String, Vec<Subscription>> = HashMap::new();
    for sub in subs {
        by_platform.entry(sub.target.clone()).or_default().push(sub);
    }
    debug!(platforms = by_platform.len(), "games-ronde");

    for (target, subs) in by_platform {
        let Some(platform) = Platform::from_target(&target) else {
            warn!(%target, "onbekend games-platform");
            continue;
        };
        let giveaways = match fetch(&app.http, platform).await {
            Ok(g) => g,
            Err(e) => {
                warn!(%target, "giveaways ophalen mislukt: {e:#}");
                continue;
            }
        };
        for sub in subs {
            if let Err(e) = process(&app, &sub, &giveaways).await {
                warn!(sub = sub.id, "games-melding mislukt: {e:#}");
            }
        }
    }
    Ok(())
}

async fn process(app: &App, sub: &Subscription, giveaways: &[Giveaway]) -> anyhow::Result<()> {
    let state = initial_state(giveaways);
    let Some(seen) = parse_seen(sub.state.as_deref()) else {
        return db::set_state(&app.db, sub.id, Some(&state)).await;
    };
    let mut result = Ok(());
    for g in new_giveaways(giveaways, &seen) {
        if let Err(e) = broadcast(app, sub, embed(g)).await {
            result = Err(e);
            break;
        }
    }
    // Net als bij RSS: altijd bijwerken, zodat een gedeeltelijk geposte ronde geen dubbels geeft.
    if sub.state.as_deref() != Some(state.as_str()) {
        db::set_state(&app.db, sub.id, Some(&state)).await?;
    }
    result
}

/// "Knightica (Epic Games) Giveaway" → "Knightica".
fn clean_title(title: &str) -> &str {
    let t = title.trim();
    let t = t.strip_suffix(" Giveaway").unwrap_or(t);
    match t.rfind(" (") {
        Some(i) if t.ends_with(')') => &t[..i],
        _ => t,
    }
}

/// GamerPower geeft einddatums als "2026-12-02 23:59:00" (UTC) of "N/A".
fn end_timestamp(end_date: &str) -> Option<i64> {
    NaiveDateTime::parse_from_str(end_date, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|n| Utc.from_utc_datetime(&n).timestamp())
}

fn embed(g: &Giveaway) -> serenity::CreateEmbed {
    let (winkel, kleur) = if g.platforms.contains("Steam") {
        ("Steam", 0x1B2838)
    } else if g.platforms.contains("Epic") {
        ("Epic Games Store", 0x2A2A2A)
    } else {
        ("Gratis game", BROODKAST_KLEUR)
    };
    let mut embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(format!(
            "🎁 Gratis op {winkel}"
        )))
        .title(truncate(clean_title(&g.title), 250))
        .url(&g.open_giveaway_url)
        .description(truncate(g.description.trim(), 300))
        .image(&g.image)
        .colour(kleur)
        .footer(serenity::CreateEmbedFooter::new(
            "🎮 Broodkast · Gratis games · via GamerPower",
        ));
    if !g.worth.is_empty() && g.worth != "N/A" {
        embed = embed.field("Normaal", format!("~~{}~~ → **gratis**", g.worth), true);
    }
    if let Some(einde) = end_timestamp(&g.end_date) {
        embed = embed.field("Gratis tot", format!("<t:{einde}:f> (<t:{einde}:R>)"), true);
    }
    embed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn giveaway(id: u64) -> Giveaway {
        Giveaway {
            id,
            title: format!("Game {id} (Steam) Giveaway"),
            worth: "$9.99".into(),
            image: String::new(),
            description: String::new(),
            open_giveaway_url: String::new(),
            end_date: "N/A".into(),
            platforms: "PC, Steam".into(),
            status: "Active".into(),
        }
    }

    #[test]
    fn nieuwe_giveaways_oudste_eerst() {
        let lijst = vec![giveaway(30), giveaway(10), giveaway(20)];
        let seen = HashSet::from([20]);
        let ids: Vec<u64> = new_giveaways(&lijst, &seen).iter().map(|g| g.id).collect();
        assert_eq!(ids, [10, 30]);
        let alles = parse_seen(Some(&initial_state(&lijst))).unwrap();
        assert!(new_giveaways(&lijst, &alles).is_empty());
    }

    #[test]
    fn titels_opschonen() {
        assert_eq!(clean_title("Knightica (Epic Games) Giveaway"), "Knightica");
        assert_eq!(clean_title("Portal 2 (Steam) Giveaway"), "Portal 2");
        assert_eq!(clean_title("Gewoon een game"), "Gewoon een game");
    }

    #[test]
    fn einddatum_lezen() {
        assert_eq!(end_timestamp("N/A"), None);
        assert_eq!(end_timestamp("2026-12-02 23:59:00"), Some(1_796_255_940));
    }
}
