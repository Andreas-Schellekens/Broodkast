use poise::serenity_prelude as serenity;

use crate::{Context, Error, integrations::BROODKAST_KLEUR};

/// Info en profielfoto van iemand (of van jezelf)
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn gebruiker(
    ctx: Context<'_>,
    #[description = "Over wie? (standaard jijzelf)"] wie: Option<serenity::User>,
) -> Result<(), Error> {
    let user = wie.unwrap_or_else(|| ctx.author().clone());

    // Serverinfo (bijnaam, lid sinds, rollen) kan alleen als de bot zelf in de server zit.
    let lid = match ctx.guild_id() {
        Some(g) if ctx.cache().guild(g).is_some() => g.member(ctx, user.id).await.ok(),
        _ => None,
    };

    let naam = user.global_name.as_deref().unwrap_or(&user.name);
    let avatar = lid
        .as_ref()
        .and_then(|l| l.avatar_url())
        .unwrap_or_else(|| user.face());

    let mut embed = serenity::CreateEmbed::new()
        .title(if user.bot {
            format!("🤖 {naam}")
        } else {
            naam.to_owned()
        })
        .description(format!("<@{}> · `{}`", user.id, user.name))
        .thumbnail(&avatar)
        .field(
            "Account gemaakt",
            format!("<t:{}:D>", user.id.created_at().unix_timestamp()),
            true,
        )
        .footer(serenity::CreateEmbedFooter::new(format!("ID: {}", user.id)))
        .colour(BROODKAST_KLEUR);

    if let Some(lid) = &lid {
        if let Some(sinds) = lid.joined_at {
            embed = embed.field(
                "Lid sinds",
                format!("<t:{}:D>", sinds.unix_timestamp()),
                true,
            );
        }
        if let Some(bijnaam) = &lid.nick {
            embed = embed.field("Bijnaam", bijnaam, true);
        }
        if !lid.roles.is_empty() {
            let rollen: Vec<String> = lid
                .roles
                .iter()
                .take(15)
                .map(|r| format!("<@&{r}>"))
                .collect();
            let meer = lid.roles.len().saturating_sub(15);
            let mut tekst = rollen.join(" ");
            if meer > 0 {
                tekst.push_str(&format!(" en nog {meer}"));
            }
            embed = embed.field(format!("Rollen ({})", lid.roles.len()), tekst, false);
        }
    }
    embed = embed.field(
        "Profielfoto",
        format!("[Volledige grootte]({avatar})"),
        false,
    );

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}
