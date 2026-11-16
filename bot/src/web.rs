//! Kleine webserver: `GET /health` voor Railway en `POST /webhook/{token}` voor inkomende meldingen.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    Router,
    body::Bytes,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    routing::{get, post},
};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use tracing::{info, warn};

use crate::{
    app::App,
    db,
    integrations::{BROODKAST_KLEUR, broadcast},
    util::truncate,
};

const MAX_BODY: usize = 16 * 1024;
/// Minimale tijd tussen twee berichten via dezelfde webhook.
const MIN_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Clone)]
struct WebState {
    app: Arc<App>,
    last_seen: Arc<Mutex<HashMap<String, Instant>>>,
}

pub async fn serve(app: Arc<App>, port: u16) -> anyhow::Result<()> {
    let state = WebState {
        app,
        last_seen: Arc::default(),
    };
    let router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/webhook/{token}", post(webhook))
        .layer(DefaultBodyLimit::max(MAX_BODY))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    info!(port, "webserver luistert");
    axum::serve(listener, router).await?;
    Ok(())
}

/// Wat je naar een webhook kunt sturen. Alle velden zijn optioneel, maar er moet iets in staan.
/// Een body die geen JSON is, wordt als platte tekst gebruikt.
#[derive(Deserialize, Default)]
struct Payload {
    /// Ook `content` (zoals bij Discord-webhooks) wordt geaccepteerd.
    #[serde(alias = "content", alias = "text")]
    message: Option<String>,
    title: Option<String>,
    url: Option<String>,
    image: Option<String>,
    color: Option<u32>,
}

impl Payload {
    fn from_body(body: &[u8]) -> Option<Self> {
        let payload = serde_json::from_slice::<Payload>(body).unwrap_or_else(|_| Payload {
            message: std::str::from_utf8(body).ok().map(str::to_owned),
            ..Default::default()
        });
        let has_text = |s: &Option<String>| s.as_deref().is_some_and(|s| !s.trim().is_empty());
        (has_text(&payload.message) || has_text(&payload.title)).then_some(payload)
    }
}

fn is_http_url(s: &str) -> bool {
    s.starts_with("https://") || s.starts_with("http://")
}

async fn webhook(
    State(state): State<WebState>,
    Path(token): Path<String>,
    body: Bytes,
) -> (StatusCode, &'static str) {
    let sub = match db::find_webhook(&state.app.db, &token).await {
        Ok(Some(sub)) => sub,
        Ok(None) => return (StatusCode::NOT_FOUND, "onbekende webhook"),
        Err(e) => {
            warn!("webhook opzoeken mislukt: {e:#}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "interne fout");
        }
    };

    {
        let mut last = state.last_seen.lock().expect("mutex niet vergiftigd");
        let now = Instant::now();
        if last
            .get(&token)
            .is_some_and(|t| now.duration_since(*t) < MIN_INTERVAL)
        {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                "rustig aan, max 1 bericht per 2 seconden",
            );
        }
        last.insert(token, now);
        // Opruimen zodat de map niet eindeloos groeit.
        last.retain(|_, t| now.duration_since(*t) < Duration::from_secs(60));
    }

    let Some(payload) = Payload::from_body(&body) else {
        return (StatusCode::BAD_REQUEST, "stuur een bericht of titel mee");
    };

    let mut embed = serenity::CreateEmbed::new()
        .colour(payload.color.unwrap_or(BROODKAST_KLEUR))
        .footer(serenity::CreateEmbedFooter::new(format!(
            "🪝 Broodkast · {}",
            truncate(sub.display_name(), 100)
        )))
        .timestamp(serenity::Timestamp::now());
    if let Some(title) = payload.title.filter(|t| !t.trim().is_empty()) {
        embed = embed.title(truncate(&title, 250));
    }
    if let Some(message) = payload.message.filter(|m| !m.trim().is_empty()) {
        embed = embed.description(truncate(&message, 4000));
    }
    if let Some(url) = payload.url.filter(|u| is_http_url(u)) {
        embed = embed.url(url);
    }
    if let Some(image) = payload.image.filter(|u| is_http_url(u)) {
        embed = embed.image(image);
    }

    match broadcast(&state.app, &sub, embed).await {
        Ok(()) => (StatusCode::NO_CONTENT, ""),
        Err(e) => {
            warn!(sub = sub.id, "webhook-melding mislukt: {e:#}");
            (StatusCode::BAD_GATEWAY, "kon niet posten in Discord")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Payload;

    #[test]
    fn json_en_tekst() {
        let p = Payload::from_body(br#"{"content":"hoi","title":"T"}"#).unwrap();
        assert_eq!(p.message.as_deref(), Some("hoi"));
        assert_eq!(p.title.as_deref(), Some("T"));

        let p = Payload::from_body(b"gewoon tekst").unwrap();
        assert_eq!(p.message.as_deref(), Some("gewoon tekst"));

        assert!(Payload::from_body(b"{}").is_none());
        assert!(Payload::from_body(b"   ").is_none());
    }
}
