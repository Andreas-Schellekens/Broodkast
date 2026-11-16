use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// Discord laat bulk-verwijderen alleen toe voor berichten jonger dan 14 dagen.
const MAX_LEEFTIJD_SECS: i64 = 14 * 24 * 3600 - 60;

/// Verwijder de laatste berichten in dit kanaal
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_MESSAGES",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES | READ_MESSAGE_HISTORY",
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn opruimen(
    ctx: Context<'_>,
    #[description = "Hoeveel berichten (1-100)"]
    #[min = 1]
    #[max = 100]
    aantal: u8,
    #[description = "Alleen berichten van dit lid"] van: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let kanaal = ctx.channel_id();
    let grens = chrono::Utc::now().timestamp() - MAX_LEEFTIJD_SECS;

    let berichten = kanaal
        .messages(ctx, serenity::GetMessages::new().limit(100))
        .await?;
    let mut te_oud = false;
    let ids: Vec<serenity::MessageId> = berichten
        .iter()
        .filter(|m| van.as_ref().is_none_or(|u| m.author.id == u.id))
        .take(aantal as usize)
        .filter(|m| {
            let jong = m.timestamp.unix_timestamp() > grens;
            te_oud |= !jong;
            jong
        })
        .map(|m| m.id)
        .collect();

    match ids.len() {
        0 => {}
        1 => kanaal.delete_message(ctx, ids[0]).await?,
        _ => kanaal.delete_messages(ctx, &ids).await?,
    }

    let mut tekst = match ids.len() {
        0 => "Niets gevonden om op te ruimen.".to_owned(),
        1 => "🧹 1 bericht opgeruimd.".to_owned(),
        n => format!("🧹 {n} berichten opgeruimd."),
    };
    if te_oud {
        tekst.push_str(" Berichten ouder dan 14 dagen kan Discord niet in bulk verwijderen.");
    }
    ctx.say(tekst).await?;
    Ok(())
}
