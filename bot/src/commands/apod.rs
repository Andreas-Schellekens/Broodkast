use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serde::Deserialize;

use crate::{Context, Error};

const APOD_URL: &str = "https://api.nasa.gov/planetary/apod";
/// Discord staat maximaal 4096 tekens toe in een embed-beschrijving.
const MAX_BESCHRIJVING: usize = 4000;

#[derive(Deserialize)]
struct Apod {
    title: String,
    explanation: String,
    date: String,
    media_type: String,
    url: String,
    hdurl: Option<String>,
    copyright: Option<String>,
}

/// Astronomiefoto van de dag (NASA)
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn apod(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let apod: Apod = ctx
        .data()
        .http
        .get(APOD_URL)
        .query(&[("api_key", ctx.data().nasa_api_key.as_str())])
        .send()
        .await
        .context("NASA API niet bereikbaar")?
        .error_for_status()
        .context("NASA API gaf een fout")?
        .json()
        .await
        .context("NASA API gaf onverwachte data")?;

    let mut beschrijving: String = apod.explanation.chars().take(MAX_BESCHRIJVING).collect();
    let bron = match &apod.copyright {
        Some(c) => format!("© {} | {}", c.trim(), apod.date),
        None => format!("NASA | {}", apod.date),
    };

    let mut embed = serenity::CreateEmbed::new()
        .title(&apod.title)
        .footer(serenity::CreateEmbedFooter::new(bron))
        .colour(0x0B3D91);

    if apod.media_type == "image" {
        // Discord laadt de afbeelding zelf; geen download of resize nodig.
        embed = embed
            .url(apod.hdurl.as_deref().unwrap_or(&apod.url))
            .image(&apod.url);
    } else {
        beschrijving.push_str(&format!("\n\n[Bekijk de video]({})", apod.url));
    }

    ctx.send(poise::CreateReply::default().embed(embed.description(beschrijving)))
        .await?;
    Ok(())
}
