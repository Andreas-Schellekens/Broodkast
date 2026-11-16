//! Integraties: bronnen die meldingen naar Discord-kanalen uitzenden.
//!
//! - Pollende integraties (`rss`, `twitch`) draaien als achtergrondtaak met een vast interval.
//! - `webhook` is push-gebaseerd en wordt bediend door de webserver in [`crate::web`].
//!
//! Een nieuwe integratie toevoegen: maak een module, voeg een [`Kind`] toe (en de CHECK in de
//! migratie), start de poller in [`spawn_pollers`] en voeg een subcommando toe aan `/volg`.

pub mod rss;
pub mod twitch;

use std::{sync::Arc, time::Duration};

use poise::serenity_prelude as serenity;
use tracing::{info, warn};

use crate::{
    app::App,
    db::{self, Subscription},
};

pub fn spawn_pollers(app: &Arc<App>) {
    spawn_poller(app.clone(), "rss", Duration::from_secs(10 * 60), rss::poll);
    if app.twitch.is_some() {
        spawn_poller(
            app.clone(),
            "twitch",
            Duration::from_secs(2 * 60),
            twitch::poll,
        );
    } else {
        info!("Twitch-integratie uit (TWITCH_CLIENT_ID/SECRET niet gezet)");
    }
}

fn spawn_poller<F, Fut>(app: Arc<App>, name: &'static str, every: Duration, poll: F)
where
    F: Fn(Arc<App>) -> Fut + Send + 'static,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(every);
        // Als een ronde uitloopt, niet alle gemiste rondes inhalen.
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            interval.tick().await;
            if let Err(e) = poll(app.clone()).await {
                warn!(integration = name, "pollronde mislukt: {e:#}");
            }
        }
    });
}

/// Stuurt een melding voor een abonnement naar zijn kanaal, met optionele rol-ping.
///
/// Als het kanaal niet meer bestaat, wordt het abonnement automatisch opgeruimd.
pub async fn broadcast(
    app: &App,
    sub: &Subscription,
    embed: serenity::CreateEmbed,
) -> anyhow::Result<()> {
    let mut message = serenity::CreateMessage::new().embed(embed);
    let mut mentions = serenity::CreateAllowedMentions::new();
    if let Some(role) = sub.ping_role_id {
        message = message.content(format!("<@&{role}>"));
        mentions = mentions.roles(vec![role]);
    }
    message = message.allowed_mentions(mentions);

    match sub.channel_id.send_message(&app.discord, message).await {
        Ok(_) => Ok(()),
        Err(e) if discord_error_code(&e) == Some(UNKNOWN_CHANNEL) => {
            warn!(sub = sub.id, guild = %sub.guild_id, "kanaal bestaat niet meer, abonnement verwijderd");
            db::delete_channel(&app.db, sub.channel_id).await?;
            Ok(())
        }
        Err(e) => Err(anyhow::Error::new(e).context(format!(
            "kon niet posten in kanaal {} (abonnement {})",
            sub.channel_id, sub.id
        ))),
    }
}

const UNKNOWN_CHANNEL: isize = 10003;

fn discord_error_code(e: &serenity::Error) -> Option<isize> {
    match e {
        serenity::Error::Http(serenity::HttpError::UnsuccessfulRequest(resp)) => {
            Some(resp.error.code)
        }
        _ => None,
    }
}

/// De merkkleur van Broodkast (korstbruin), voor embeds zonder eigen kleur.
pub const BROODKAST_KLEUR: u32 = 0xC8873A;
