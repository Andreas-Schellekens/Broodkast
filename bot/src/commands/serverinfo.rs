use anyhow::Context as _;
use poise::serenity_prelude as serenity;

use crate::{Context, Error, integrations::BROODKAST_KLEUR};

/// Info over deze server
#[poise::command(
    slash_command,
    guild_only,
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    // Alles uit de cache halen vóór de eerste await: de cache-referentie is niet Send.
    let embed = {
        let guild = ctx
            .guild()
            .context("server niet in de cache (zit Broodkast in deze server?)")?;
        let tel = |soort: serenity::ChannelType| {
            guild.channels.values().filter(|c| c.kind == soort).count()
        };
        let tekst = tel(serenity::ChannelType::Text) + tel(serenity::ChannelType::News);
        let spraak = tel(serenity::ChannelType::Voice) + tel(serenity::ChannelType::Stage);
        let niveau = match guild.premium_tier {
            serenity::PremiumTier::Tier1 => 1,
            serenity::PremiumTier::Tier2 => 2,
            serenity::PremiumTier::Tier3 => 3,
            _ => 0,
        };
        let boosts = guild.premium_subscription_count.unwrap_or(0);
        let gemaakt = guild.id.created_at().unix_timestamp();

        let mut embed = serenity::CreateEmbed::new()
            .title(&guild.name)
            .field("Eigenaar", format!("<@{}>", guild.owner_id), true)
            .field("Leden", guild.member_count.to_string(), true)
            .field("Aangemaakt", format!("<t:{gemaakt}:D>"), true)
            .field("Kanalen", format!("💬 {tekst} · 🔊 {spraak}"), true)
            .field(
                "Rollen",
                guild.roles.len().saturating_sub(1).to_string(),
                true,
            )
            .field("Boosts", format!("{boosts} (niveau {niveau})"), true)
            .footer(serenity::CreateEmbedFooter::new(format!(
                "ID: {}",
                guild.id
            )))
            .colour(BROODKAST_KLEUR);
        if let Some(beschrijving) = &guild.description {
            embed = embed.description(beschrijving);
        }
        if let Some(icon) = guild.icon_url() {
            embed = embed.thumbnail(icon);
        }
        embed
    };
    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}
