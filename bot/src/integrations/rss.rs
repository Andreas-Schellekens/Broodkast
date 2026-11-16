//! RSS/Atom/JSON-feeds. Dekt ook YouTube-kanalen, Reddit, GitHub-releases, blogs, nieuwssites, ...

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::Context as _;
use feed_rs::model::{Entry, Feed};
use futures::StreamExt as _;
use poise::serenity_prelude as serenity;
use tracing::{debug, warn};

use crate::{
    app::App,
    db::{self, Kind, Subscription},
    integrations::{BROODKAST_KLEUR, broadcast},
    util::{check_public_url, fetch_limited, strip_html, truncate},
};

const MAX_FEED_BYTES: usize = 5 * 1024 * 1024;
/// Alleen de bovenste items van een feed tellen mee; oudere items worden nooit "nieuw".
const WINDOW: usize = 50;
/// Maximaal aantal meldingen per feed per ronde, tegen spam bij een plots volle feed.
const MAX_PER_POLL: usize = 5;
const CONCURRENT_FETCHES: usize = 8;

pub async fn fetch(http: &reqwest::Client, url: &str) -> anyhow::Result<Feed> {
    let url = check_public_url(url)?;
    let body = fetch_limited(http, url, MAX_FEED_BYTES).await?;
    feed_rs::parser::parse(body.as_slice()).context("dit is geen geldige RSS-, Atom- of JSON-feed")
}

/// De toestand die we per abonnement opslaan: ID's van items die al gezien zijn.
pub fn initial_state(feed: &Feed) -> String {
    serialize_seen(feed.entries.iter().take(WINDOW).map(|e| e.id.as_str()))
}

fn serialize_seen<'a>(ids: impl Iterator<Item = &'a str>) -> String {
    serde_json::to_string(&ids.collect::<Vec<_>>()).expect("strings serialiseren faalt niet")
}

fn parse_seen(state: Option<&str>) -> Option<HashSet<String>> {
    serde_json::from_str::<Vec<String>>(state?)
        .ok()
        .map(HashSet::from_iter)
}

/// Nieuwe items in de volgorde waarin ze gepost moeten worden (oudste eerst).
fn new_entries<'a>(feed: &'a Feed, seen: &HashSet<String>) -> Vec<&'a Entry> {
    let mut fresh: Vec<&Entry> = feed
        .entries
        .iter()
        .take(WINDOW)
        .filter(|e| !seen.contains(&e.id))
        .collect();
    // Feeds staan meestal nieuw→oud, maar niet altijd. Sorteer op datum waar die er is.
    fresh.sort_by_key(|e| std::cmp::Reverse(e.published.or(e.updated)));
    fresh.truncate(MAX_PER_POLL);
    fresh.reverse();
    fresh
}

pub async fn poll(app: Arc<App>) -> anyhow::Result<()> {
    let subs = db::list_by_kind(&app.db, Kind::Rss).await?;
    let mut by_url: HashMap<String, Vec<Subscription>> = HashMap::new();
    for sub in subs {
        by_url.entry(sub.target.clone()).or_default().push(sub);
    }
    debug!(feeds = by_url.len(), "RSS-ronde");

    futures::stream::iter(by_url)
        .for_each_concurrent(CONCURRENT_FETCHES, |(url, subs)| {
            let app = app.clone();
            async move {
                match fetch(&app.http, &url).await {
                    Ok(feed) => {
                        for sub in subs {
                            if let Err(e) = process(&app, &sub, &feed).await {
                                warn!(sub = sub.id, "RSS-melding mislukt: {e:#}");
                            }
                        }
                    }
                    Err(e) => warn!(%url, "feed ophalen mislukt: {e:#}"),
                }
            }
        })
        .await;
    Ok(())
}

async fn process(app: &App, sub: &Subscription, feed: &Feed) -> anyhow::Result<()> {
    let Some(seen) = parse_seen(sub.state.as_deref()) else {
        // Geen (geldige) toestand: alles als gezien markeren in plaats van de feed te spammen.
        return db::set_state(&app.db, sub.id, Some(&initial_state(feed))).await;
    };

    let mut result = Ok(());
    for entry in new_entries(feed, &seen) {
        if let Err(e) = broadcast(app, sub, embed(sub, feed, entry)).await {
            result = Err(e);
            break;
        }
    }

    // Altijd de toestand bijwerken, ook na een fout: anders zouden items die wél gepost
    // zijn de volgende ronde opnieuw gepost worden. Een mislukte melding wordt niet herhaald.
    let state = initial_state(feed);
    if sub.state.as_deref() != Some(state.as_str()) {
        db::set_state(&app.db, sub.id, Some(&state)).await?;
    }
    result
}

fn embed(sub: &Subscription, feed: &Feed, entry: &Entry) -> serenity::CreateEmbed {
    let feed_title = sub
        .label
        .clone()
        .or_else(|| feed.title.as_ref().map(|t| t.content.clone()))
        .unwrap_or_else(|| sub.target.clone());
    let title = entry
        .title
        .as_ref()
        .map(|t| strip_html(&t.content))
        .unwrap_or_else(|| "Nieuw bericht".to_owned());

    let mut embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(truncate(&feed_title, 250)))
        .title(truncate(&title, 250))
        .colour(BROODKAST_KLEUR)
        .footer(serenity::CreateEmbedFooter::new("📰 Broodkast · RSS"));

    if let Some(link) = entry.links.first() {
        embed = embed.url(&link.href);
    }
    let summary = entry
        .summary
        .as_ref()
        .map(|s| s.content.as_str())
        .or_else(|| entry.content.as_ref().and_then(|c| c.body.as_deref()))
        .or_else(|| {
            entry
                .media
                .first()
                .and_then(|m| m.description.as_ref())
                .map(|d| d.content.as_str())
        });
    if let Some(summary) = summary {
        let text = strip_html(summary);
        if !text.is_empty() {
            embed = embed.description(truncate(&text, 350));
        }
    }
    if let Some(thumb) = entry.media.iter().flat_map(|m| &m.thumbnails).next() {
        embed = embed.image(&thumb.image.uri);
    }
    if let Some(when) = entry.published.or(entry.updated) {
        embed = embed.timestamp(when);
    }
    embed
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEED: &str = r#"<?xml version="1.0"?>
<rss version="2.0"><channel><title>Bakkerij</title>
<item><guid>c</guid><title>Derde</title><pubDate>Wed, 03 Jan 2024 10:00:00 GMT</pubDate></item>
<item><guid>b</guid><title>Tweede</title><pubDate>Tue, 02 Jan 2024 10:00:00 GMT</pubDate></item>
<item><guid>a</guid><title>Eerste</title><pubDate>Mon, 01 Jan 2024 10:00:00 GMT</pubDate></item>
</channel></rss>"#;

    fn feed() -> Feed {
        feed_rs::parser::parse(FEED.as_bytes()).unwrap()
    }

    fn titles(entries: &[&Entry]) -> Vec<String> {
        entries
            .iter()
            .map(|e| e.title.as_ref().unwrap().content.clone())
            .collect()
    }

    #[test]
    fn alles_gezien_geeft_niets() {
        let feed = feed();
        let seen = parse_seen(Some(&initial_state(&feed))).unwrap();
        assert!(new_entries(&feed, &seen).is_empty());
    }

    #[test]
    fn nieuwe_items_oudste_eerst() {
        let feed = feed();
        let seen = HashSet::from(["a".to_owned()]);
        assert_eq!(titles(&new_entries(&feed, &seen)), ["Tweede", "Derde"]);
    }

    #[test]
    fn ongeldige_toestand_is_none() {
        assert!(parse_seen(None).is_none());
        assert!(parse_seen(Some("kapot")).is_none());
    }
}
