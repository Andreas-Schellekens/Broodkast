use poise::serenity_prelude as serenity;

use crate::{Context, Error, integrations::BROODKAST_KLEUR};

/// Wat is Broodkast?
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn over(ctx: Context<'_>) -> Result<(), Error> {
    let bot_id = ctx.framework().bot_id();
    let invite = format!("https://discord.com/oauth2/authorize?client_id={bot_id}");
    let servers = ctx.cache().guild_count();

    let embed = serenity::CreateEmbed::new()
        .title("🍞📻 Broodkast")
        .description(
            "De Nederlandstalige bot die uitzendt wat jij volgt: feeds, Twitch-streams en \
             webhooks, vers in je kanaal. En een koe die praat.",
        )
        .field("Versie", env!("CARGO_PKG_VERSION"), true)
        .field("Servers", servers.to_string(), true)
        .field(
            "Uitnodigen",
            format!("[Voeg Broodkast toe]({invite})"),
            true,
        )
        .colour(BROODKAST_KLEUR);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
