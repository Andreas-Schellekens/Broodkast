//! `/volg`, `/ontvolg` en `/volglijst`: abonnementen op integraties beheren.

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use rand::distr::{Alphanumeric, SampleString};

use crate::{
    Context, Error,
    db::{self, Kind, NewSubscription},
    integrations::{BROODKAST_KLEUR, games, rss, twitch},
    util::truncate,
};

/// Maximaal aantal abonnementen per server, zodat één server de bot niet kan overspoelen.
const MAX_PER_GUILD: i64 = 50;

/// Stuur meldingen van een dienst naar een kanaal
#[poise::command(
    slash_command,
    subcommands("volg_rss", "volg_nieuws", "volg_twitch", "volg_games", "volg_webhook"),
    subcommand_required,
    guild_only,
    default_member_permissions = "MANAGE_GUILD",
    required_permissions = "MANAGE_GUILD",
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn volg(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Volg een RSS- of Atom-feed (ook YouTube-kanalen, Reddit, GitHub-releases, ...)
#[poise::command(slash_command, rename = "rss")]
async fn volg_rss(
    ctx: Context<'_>,
    #[description = "URL van de feed"] url: String,
    #[description = "Kanaal voor de meldingen (standaard dit kanaal)"]
    #[channel_types("Text", "News")]
    kanaal: Option<serenity::GuildChannel>,
    #[description = "Rol om te pingen bij een nieuw bericht"] rol: Option<serenity::Role>,
    #[description = "Naam om te tonen (standaard de titel van de feed)"] naam: Option<String>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    add_feed(ctx, url.trim(), naam, kanaal, rol).await
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
enum Nieuwsbron {
    #[name = "VRT NWS: alle artikels"]
    VrtNws,
    #[name = "VRT NWS: hoofdpunten"]
    VrtNwsHoofdpunten,
}

impl Nieuwsbron {
    fn url(self) -> &'static str {
        match self {
            Nieuwsbron::VrtNws => "https://www.vrt.be/vrtnws/nl.rss.articles.xml",
            Nieuwsbron::VrtNwsHoofdpunten => "https://www.vrt.be/vrtnws/nl.rss.headlines.xml",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Nieuwsbron::VrtNws => "VRT NWS",
            Nieuwsbron::VrtNwsHoofdpunten => "VRT NWS: hoofdpunten",
        }
    }
}

/// Volg een nieuwssite zonder een feed-URL te moeten zoeken
#[poise::command(slash_command, rename = "nieuws")]
async fn volg_nieuws(
    ctx: Context<'_>,
    #[description = "Welke nieuwsbron?"] bron: Nieuwsbron,
    #[description = "Kanaal voor de meldingen (standaard dit kanaal)"]
    #[channel_types("Text", "News")]
    kanaal: Option<serenity::GuildChannel>,
    #[description = "Rol om te pingen bij een nieuw artikel"] rol: Option<serenity::Role>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    add_feed(ctx, bron.url(), Some(bron.label().to_owned()), kanaal, rol).await
}

/// Gedeeld door `/volg rss` en `/volg nieuws`: feed ophalen, toestand zetten en opslaan.
async fn add_feed(
    ctx: Context<'_>,
    url: &str,
    naam: Option<String>,
    kanaal: Option<serenity::GuildChannel>,
    rol: Option<serenity::Role>,
) -> Result<(), Error> {
    let feed = match rss::fetch(&ctx.data().http, url).await {
        Ok(feed) => feed,
        Err(e) => return reply(ctx, format!("❌ Kon die feed niet lezen: {e:#}")).await,
    };
    let label = naam
        .or_else(|| feed.title.as_ref().map(|t| t.content.clone()))
        .map(|l| truncate(l.trim(), 100));
    let state = rss::initial_state(&feed);
    add(
        ctx,
        Kind::Rss,
        url,
        label.as_deref(),
        kanaal,
        rol,
        Some(&state),
    )
    .await?;
    Ok(())
}

/// Krijg een melding wanneer een game tijdelijk gratis is op Epic of Steam
#[poise::command(slash_command, rename = "gratisgames")]
async fn volg_games(
    ctx: Context<'_>,
    #[description = "Welke winkel?"] platform: games::Platform,
    #[description = "Kanaal voor de meldingen (standaard dit kanaal)"]
    #[channel_types("Text", "News")]
    kanaal: Option<serenity::GuildChannel>,
    #[description = "Rol om te pingen bij een gratis game"] rol: Option<serenity::Role>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    // Lopende giveaways meteen als gezien markeren; alleen nieuwe worden gemeld.
    let lopend = match games::fetch(&ctx.data().http, platform).await {
        Ok(g) => g,
        Err(e) => return reply(ctx, format!("❌ Kon de gratis games niet ophalen: {e:#}")).await,
    };
    let state = games::initial_state(&lopend);
    add(
        ctx,
        Kind::Games,
        platform.as_target(),
        Some(platform.label()),
        kanaal,
        rol,
        Some(&state),
    )
    .await?;
    Ok(())
}

/// Krijg een melding wanneer een Twitch-streamer live gaat
#[poise::command(slash_command, rename = "twitch")]
async fn volg_twitch(
    ctx: Context<'_>,
    #[description = "Twitch-naam of link naar het kanaal"] streamer: String,
    #[description = "Kanaal voor de meldingen (standaard dit kanaal)"]
    #[channel_types("Text", "News")]
    kanaal: Option<serenity::GuildChannel>,
    #[description = "Rol om te pingen als de stream begint"] rol: Option<serenity::Role>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let Some(client) = &ctx.data().twitch else {
        return reply(
            ctx,
            "❌ De Twitch-integratie staat nog niet aan op deze bot.",
        )
        .await;
    };
    let login = match twitch::normalize_login(&streamer) {
        Ok(login) => login,
        Err(e) => return reply(ctx, format!("❌ {e}")).await,
    };
    let Some(user) = client.user(&login).await? else {
        return reply(
            ctx,
            format!("❌ Geen Twitch-kanaal gevonden met de naam `{login}`."),
        )
        .await;
    };

    add(
        ctx,
        Kind::Twitch,
        &user.login,
        Some(&user.display_name),
        kanaal,
        rol,
        None,
    )
    .await?;
    Ok(())
}

/// Maak een webhook-URL waarmee andere diensten berichten naar een kanaal sturen
#[poise::command(slash_command, rename = "webhook")]
async fn volg_webhook(
    ctx: Context<'_>,
    #[description = "Naam voor deze webhook, bv. \"Server-monitor\""] naam: String,
    #[description = "Kanaal voor de berichten (standaard dit kanaal)"]
    #[channel_types("Text", "News")]
    kanaal: Option<serenity::GuildChannel>,
    #[description = "Rol om te pingen bij elk bericht"] rol: Option<serenity::Role>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let Some(base) = ctx.data().public_url.clone() else {
        return reply(
            ctx,
            "❌ Webhooks staan niet aan: de bot heeft geen publieke URL.",
        )
        .await;
    };
    let token = Alphanumeric.sample_string(&mut rand::rng(), 40);
    let label = truncate(naam.trim(), 100);

    if add(ctx, Kind::Webhook, &token, Some(&label), kanaal, rol, None)
        .await?
        .is_some()
    {
        let url = format!("{base}/webhook/{token}");
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "🔐 **Je webhook-URL** — houd deze geheim, iedereen met de link kan berichten sturen:\n\
                     ```\n{url}\n```\n\
                     Voorbeeld:\n\
                     ```bash\ncurl -X POST {url} -H \"Content-Type: application/json\" \\\n  \
                     -d '{{\"title\": \"Hallo\", \"message\": \"Vers uit de broodkast!\"}}'\n```\n\
                     Velden: `message` (of `content`), `title`, `url`, `image`, `color`. \
                     Platte tekst werkt ook."
                ))
                .ephemeral(true),
        )
        .await?;
    }
    Ok(())
}

async fn reply(ctx: Context<'_>, text: impl Into<String>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().content(text).ephemeral(true))
        .await?;
    Ok(())
}

/// Slaat een abonnement op en test meteen of de bot in het kanaal kan posten.
/// Geeft de ID terug, of `None` als het niet gelukt is (de gebruiker kreeg dan al uitleg).
async fn add(
    ctx: Context<'_>,
    kind: Kind,
    target: &str,
    label: Option<&str>,
    kanaal: Option<serenity::GuildChannel>,
    rol: Option<serenity::Role>,
    state: Option<&str>,
) -> Result<Option<i64>, Error> {
    let guild_id = ctx.guild_id().context("alleen in servers")?;
    let channel_id = kanaal.map(|c| c.id).unwrap_or_else(|| ctx.channel_id());
    let db = &ctx.data().db;

    if db::count_for_guild(db, guild_id).await? >= MAX_PER_GUILD {
        reply(ctx, format!("❌ Deze server heeft al {MAX_PER_GUILD} abonnementen. Verwijder er eerst een met `/ontvolg`.")).await?;
        return Ok(None);
    }

    let Some(id) = db::insert(
        db,
        NewSubscription {
            guild_id,
            channel_id,
            kind,
            target,
            label,
            ping_role_id: rol.as_ref().map(|r| r.id),
            state,
            created_by: ctx.author().id,
        },
    )
    .await?
    else {
        reply(ctx, format!("ℹ️ <#{channel_id}> volgt dit al.")).await?;
        return Ok(None);
    };

    let name = label.unwrap_or(target);
    let announce = serenity::CreateMessage::new().embed(
        serenity::CreateEmbed::new()
            .description(format!(
                "📻 Dit kanaal ontvangt nu meldingen van {} **{}**.",
                kind.emoji(),
                truncate(name, 100)
            ))
            .colour(BROODKAST_KLEUR),
    );
    if channel_id.send_message(ctx.http(), announce).await.is_err() {
        db::delete(db, guild_id, id).await?;
        reply(
            ctx,
            format!(
                "❌ Ik kan niet posten in <#{channel_id}>. Geef me daar de rechten \
                 **Kanaal bekijken**, **Berichten verzenden** en **Links insluiten**."
            ),
        )
        .await?;
        return Ok(None);
    }

    if kind != Kind::Webhook {
        let mut text =
            format!("✅ Gelukt! Abonnement `#{id}` stuurt meldingen naar <#{channel_id}>.");
        if let Some(rol) = &rol
            && !rol.mentionable
        {
            text.push_str(&format!(
                "\n⚠️ De rol **{}** is niet vermeldbaar; zet dat aan of geef mij \
                 *Iedereen vermelden*, anders krijgt niemand een ping.",
                rol.name
            ));
        }
        reply(ctx, text).await?;
    }
    Ok(Some(id))
}

async fn autocomplete_subscription(
    ctx: Context<'_>,
    partial: &str,
) -> serenity::CreateAutocompleteResponse {
    let mut choices = Vec::new();
    if let Some(guild) = ctx.guild_id()
        && let Ok(subs) = db::list_for_guild(&ctx.data().db, guild).await
    {
        let partial = partial.to_lowercase();
        let channel_names: std::collections::HashMap<_, _> = ctx
            .guild()
            .map(|g| {
                g.channels
                    .iter()
                    .map(|(id, c)| (*id, c.name.clone()))
                    .collect()
            })
            .unwrap_or_default();
        for sub in subs {
            let channel = channel_names
                .get(&sub.channel_id)
                .cloned()
                .unwrap_or_else(|| sub.channel_id.to_string());
            let label = truncate(
                &format!(
                    "#{} · {} {} → #{channel}",
                    sub.id,
                    sub.kind,
                    sub.display_name()
                ),
                100,
            );
            if label.to_lowercase().contains(&partial) {
                choices.push(serenity::AutocompleteChoice::new(label, sub.id));
            }
            if choices.len() == 25 {
                break;
            }
        }
    }
    serenity::CreateAutocompleteResponse::new().set_choices(choices)
}

/// Stop met een abonnement
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_GUILD",
    required_permissions = "MANAGE_GUILD",
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn ontvolg(
    ctx: Context<'_>,
    #[description = "Welk abonnement"]
    #[autocomplete = "autocomplete_subscription"]
    abonnement: i64,
) -> Result<(), Error> {
    let guild = ctx.guild_id().context("alleen in servers")?;
    let text = if db::delete(&ctx.data().db, guild, abonnement).await? {
        format!("🗑️ Abonnement `#{abonnement}` verwijderd.")
    } else {
        format!("❌ Geen abonnement `#{abonnement}` gevonden in deze server.")
    };
    reply(ctx, text).await
}

/// Toon alles wat deze server volgt
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_GUILD",
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn volglijst(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().context("alleen in servers")?;
    let subs = db::list_for_guild(&ctx.data().db, guild).await?;
    if subs.is_empty() {
        return reply(ctx, "Deze server volgt nog niets. Begin met `/volg`!").await;
    }

    let lines: Vec<String> = subs
        .iter()
        .map(|s| {
            let ping = s
                .ping_role_id
                .map(|r| format!(" · <@&{r}>"))
                .unwrap_or_default();
            format!(
                "`#{}` {} **{}** → <#{}>{ping}",
                s.id,
                s.kind.emoji(),
                truncate(s.display_name(), 60),
                s.channel_id
            )
        })
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title("📻 Wat de broodkast hier uitzendt")
        .description(truncate(&lines.join("\n"), 4000))
        .colour(BROODKAST_KLEUR)
        .footer(serenity::CreateEmbedFooter::new(format!(
            "{} van {MAX_PER_GUILD} abonnementen",
            subs.len()
        )));
    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .allowed_mentions(serenity::CreateAllowedMentions::new())
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
