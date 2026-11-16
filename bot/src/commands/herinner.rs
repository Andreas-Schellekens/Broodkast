//! `/herinner`: laat Broodkast je later ergens aan herinneren.
//!
//! Herinneringen staan in de database, zodat ze een herstart overleven. [`bezorg`] draait als
//! achtergrondtaak en verstuurt wat vervallen is.

use std::{sync::Arc, time::Duration};

use poise::serenity_prelude as serenity;
use tracing::warn;

use crate::{Context, Error, app::App, db, util::truncate};

/// Maximaal aantal openstaande herinneringen per gebruiker.
const MAX_PER_GEBRUIKER: i64 = 25;
const MIN_DUUR: Duration = Duration::from_secs(10);
const MAX_DUUR: Duration = Duration::from_secs(365 * 24 * 3600);

/// Laat Broodkast je later ergens aan herinneren
#[poise::command(
    slash_command,
    subcommands("nieuw", "lijst", "wis"),
    subcommand_required,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn herinner(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Stel een herinnering in
#[poise::command(slash_command)]
async fn nieuw(
    ctx: Context<'_>,
    #[description = "Wanneer, bv. 10m, 2u, 1d of 1u30m"] over: String,
    #[description = "Waaraan moet ik je herinneren?"]
    #[max_length = 500]
    wat: String,
) -> Result<(), Error> {
    let Some(duur) = parse_duur(&over) else {
        return antwoord(
            ctx,
            "❌ Die tijd snap ik niet. Probeer iets als `10m`, `2u`, `1d` of `1u30m`.",
        )
        .await;
    };
    if duur < MIN_DUUR {
        return antwoord(ctx, "❌ Dat is wel heel snel. Kies minstens 10 seconden.").await;
    }
    if duur > MAX_DUUR {
        return antwoord(
            ctx,
            "❌ Zo ver vooruit kijkt de broodkast niet: maximaal een jaar.",
        )
        .await;
    }

    let db = &ctx.data().db;
    let user = ctx.author().id;
    if db::count_reminders(db, user).await? >= MAX_PER_GEBRUIKER {
        return antwoord(
            ctx,
            format!(
                "❌ Je hebt al {MAX_PER_GEBRUIKER} herinneringen open. Wis er eerst een met \
                 `/herinner wis`."
            ),
        )
        .await;
    }

    // Alleen in het kanaal posten als de bot zelf in deze server zit. Bij user-install,
    // in DM's en groepsgesprekken kan dat niet, dus dan via DM.
    let kanaal = ctx
        .guild_id()
        .filter(|g| ctx.cache().guild(*g).is_some())
        .map(|_| ctx.channel_id());

    let nu = chrono::Utc::now().timestamp();
    let wanneer = nu + duur.as_secs() as i64;
    db::insert_reminder(db, user, kanaal, wat.trim(), wanneer, nu).await?;

    let waar = if kanaal.is_some() {
        "in dit kanaal"
    } else {
        "via DM"
    };
    antwoord(
        ctx,
        format!("⏰ Komt in orde! Ik herinner je <t:{wanneer}:R> (<t:{wanneer}:f>) {waar}."),
    )
    .await
}

/// Toon je openstaande herinneringen
#[poise::command(slash_command)]
async fn lijst(ctx: Context<'_>) -> Result<(), Error> {
    let herinneringen = db::list_reminders(&ctx.data().db, ctx.author().id).await?;
    if herinneringen.is_empty() {
        return antwoord(ctx, "Je hebt geen openstaande herinneringen.").await;
    }
    let regels: Vec<String> = herinneringen
        .iter()
        .map(|r| {
            format!(
                "`#{}` · <t:{}:R> · {}",
                r.id,
                r.due_at,
                truncate(&r.message, 80)
            )
        })
        .collect();
    antwoord(
        ctx,
        format!("⏰ **Je herinneringen**\n{}", regels.join("\n")),
    )
    .await
}

/// Wis een herinnering
#[poise::command(slash_command)]
async fn wis(
    ctx: Context<'_>,
    #[description = "Welke herinnering?"]
    #[autocomplete = "autocomplete_herinnering"]
    herinnering: i64,
) -> Result<(), Error> {
    let tekst = if db::delete_reminder(&ctx.data().db, ctx.author().id, herinnering).await? {
        format!("🗑️ Herinnering `#{herinnering}` gewist.")
    } else {
        format!("❌ Je hebt geen herinnering `#{herinnering}`.")
    };
    antwoord(ctx, tekst).await
}

async fn autocomplete_herinnering(
    ctx: Context<'_>,
    partial: &str,
) -> serenity::CreateAutocompleteResponse {
    let partial = partial.to_lowercase();
    let mut keuzes = Vec::new();
    if let Ok(lijst) = db::list_reminders(&ctx.data().db, ctx.author().id).await {
        for r in lijst {
            if !partial.is_empty() && !r.message.to_lowercase().contains(&partial) {
                continue;
            }
            let label = truncate(&format!("#{} · {}", r.id, r.message), 100);
            keuzes.push(serenity::AutocompleteChoice::new(label, r.id));
            if keuzes.len() == 25 {
                break;
            }
        }
    }
    serenity::CreateAutocompleteResponse::new().set_choices(keuzes)
}

async fn antwoord(ctx: Context<'_>, tekst: impl Into<String>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(tekst)
            .ephemeral(true)
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}

/// Verstuurt alle vervallen herinneringen. Draait periodiek als achtergrondtaak.
pub async fn bezorg(app: Arc<App>) -> anyhow::Result<()> {
    let nu = chrono::Utc::now().timestamp();
    for r in db::due_reminders(&app.db, nu).await? {
        // Eerst wissen: een herinnering die niet aankomt, mag niet elke ronde opnieuw proberen.
        db::delete_reminder_by_id(&app.db, r.id).await?;
        if let Err(e) = verstuur(&app, &r).await {
            warn!(herinnering = r.id, "kon herinnering niet bezorgen: {e:#}");
        }
    }
    Ok(())
}

async fn verstuur(app: &App, r: &db::Reminder) -> anyhow::Result<()> {
    let bericht = |tekst: &str| {
        serenity::CreateMessage::new()
            .content(tekst)
            .allowed_mentions(serenity::CreateAllowedMentions::new().users(vec![r.user_id]))
    };
    let tekst = format!(
        "⏰ <@{}>, je vroeg me <t:{}:R> om je hieraan te herinneren:\n>>> {}",
        r.user_id, r.created_at, r.message
    );

    if let Some(kanaal) = r.channel_id {
        match kanaal.send_message(&app.discord, bericht(&tekst)).await {
            Ok(_) => return Ok(()),
            Err(e) => warn!(herinnering = r.id, "kanaal lukte niet, probeer DM: {e}"),
        }
    }
    let dm = r.user_id.create_dm_channel(&app.discord).await?;
    dm.id.send_message(&app.discord, bericht(&tekst)).await?;
    Ok(())
}

/// Leest een duur zoals `10m`, `2u`, `1d`, `1u30m`, `90 minuten` of `1 uur 15 min`.
/// Een los getal telt als minuten.
pub fn parse_duur(input: &str) -> Option<Duration> {
    let tekst: String = input
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if tekst.is_empty() {
        return None;
    }
    if let Ok(minuten) = tekst.parse::<u64>() {
        return minuten.checked_mul(60).map(Duration::from_secs);
    }

    let mut totaal: u64 = 0;
    let mut rest = tekst.as_str();
    while !rest.is_empty() {
        let cijfers = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if cijfers == 0 {
            return None;
        }
        let getal: u64 = rest[..cijfers].parse().ok()?;
        rest = &rest[cijfers..];
        let letters = rest
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(rest.len());
        let factor = match &rest[..letters] {
            "s" | "sec" | "seconde" | "seconden" => 1,
            "m" | "min" | "minuut" | "minuten" => 60,
            "u" | "h" | "uur" | "uren" => 3600,
            "d" | "dag" | "dagen" => 86_400,
            "w" | "week" | "weken" => 604_800,
            _ => return None,
        };
        rest = &rest[letters..];
        totaal = totaal.checked_add(getal.checked_mul(factor)?)?;
    }
    (totaal > 0).then(|| Duration::from_secs(totaal))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secs(input: &str) -> Option<u64> {
        parse_duur(input).map(|d| d.as_secs())
    }

    #[test]
    fn leest_eenvoudige_duren() {
        assert_eq!(secs("10m"), Some(600));
        assert_eq!(secs("2u"), Some(7200));
        assert_eq!(secs("2h"), Some(7200));
        assert_eq!(secs("1d"), Some(86_400));
        assert_eq!(secs("1w"), Some(604_800));
        assert_eq!(secs("45s"), Some(45));
        assert_eq!(secs("15"), Some(900));
    }

    #[test]
    fn leest_samengestelde_duren() {
        assert_eq!(secs("1u30m"), Some(5400));
        assert_eq!(secs("1 uur 15 min"), Some(4500));
        assert_eq!(secs("2 dagen 3 uren"), Some(2 * 86_400 + 3 * 3600));
        assert_eq!(secs("90 Minuten"), Some(5400));
    }

    #[test]
    fn weigert_onzin() {
        assert_eq!(secs(""), None);
        assert_eq!(secs("morgen"), None);
        assert_eq!(secs("10x"), None);
        assert_eq!(secs("u10"), None);
        assert_eq!(secs("0m"), None);
        assert_eq!(secs("99999999999999999999d"), None);
    }
}
