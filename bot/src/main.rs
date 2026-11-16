mod app;
mod commands;
mod config;
mod db;
mod integrations;
mod util;
mod web;

use std::{sync::Arc, time::Duration};

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use tracing::{error, info, warn};

pub use crate::app::Data;
use crate::{app::App, config::Config, integrations::twitch::TwitchClient};

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!(command = %ctx.command().qualified_name, "commando mislukt: {error:?}");
            let _ = ctx
                .send(
                    poise::CreateReply::default()
                        .content(
                            "Oei, er ging iets mis in de broodkast. Probeer het later opnieuw.",
                        )
                        .ephemeral(true),
                )
                .await;
        }
        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                error!("fout tijdens foutafhandeling: {e}");
            }
        }
    }
}

/// Ruimt abonnementen op als de bot uit een server gezet wordt of een kanaal verdwijnt.
async fn on_event(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    let db = &framework.user_data.db;
    match event {
        serenity::FullEvent::GuildDelete { incomplete, .. } if !incomplete.unavailable => {
            let n = db::delete_guild(db, incomplete.id).await?;
            info!(guild = %incomplete.id, verwijderd = n, "uit server verwijderd");
        }
        serenity::FullEvent::ChannelDelete { channel, .. } => {
            db::delete_channel(db, channel.id).await?;
        }
        _ => {}
    }
    Ok(())
}

/// HTTP-client voor door gebruikers opgegeven URL's: redirects naar interne adressen worden
/// niet gevolgd, en trage servers worden afgekapt.
fn http_client() -> anyhow::Result<reqwest::Client> {
    let redirects = reqwest::redirect::Policy::custom(|attempt| {
        if attempt.previous().len() >= 5 {
            attempt.error("te veel redirects")
        } else if util::check_public_url(attempt.url().as_str()).is_err() {
            attempt.error("redirect naar een niet-toegestaan adres")
        } else {
            attempt.follow()
        }
    });
    Ok(reqwest::Client::builder()
        .user_agent(concat!(
            "Broodkast/",
            env!("CARGO_PKG_VERSION"),
            " (+https://github.com/Andreas-Schellekens/Broodkast)"
        ))
        .redirect(redirects)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(20))
        .build()?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env is optioneel: in productie komen de variabelen gewoon uit de omgeving.
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,serenity=warn".into()),
        )
        .init();

    let config = Config::from_env()?;
    let http = http_client()?;
    let app = Arc::new(App {
        db: db::connect(&config.database_url).await?,
        twitch: config.twitch.map(|t| TwitchClient::new(http.clone(), t)),
        http,
        discord: Arc::new(serenity::Http::new(&config.discord_token)),
        nasa_api_key: config.nasa_api_key,
        public_url: config.public_url,
    });
    info!(database = %config.database_url, "database klaar");

    // Webserver (health check + webhooks) en pollers draaien los van de gateway.
    let web_app = app.clone();
    let port = config.port;
    tokio::spawn(async move {
        if let Err(e) = web::serve(web_app, port).await {
            error!("webserver gestopt: {e:#}");
        }
    });
    if app.public_url.is_none() {
        warn!("geen PUBLIC_URL: /volg webhook staat uit");
    }
    integrations::spawn_pollers(&app);
    integrations::spawn_poller(
        app.clone(),
        "herinneringen",
        Duration::from_secs(30),
        commands::herinner::bezorg,
    );

    let dev_guild_id = config.dev_guild_id;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            on_error: |error| Box::pin(on_error(error)),
            event_handler: |framework, event| Box::pin(on_event(framework, event)),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                match dev_guild_id {
                    // Guild-registratie is meteen zichtbaar, handig tijdens ontwikkeling.
                    Some(guild) => {
                        poise::builtins::register_in_guild(ctx, commands, guild).await?;
                        info!(%guild, "commando's geregistreerd in testserver");
                    }
                    // Globale registratie kan tot een uur duren voor Discord ze overal toont.
                    None => {
                        poise::builtins::register_globally(ctx, commands).await?;
                        info!("commando's globaal geregistreerd");
                    }
                }
                info!(
                    "ingelogd als {} — de broodkast zendt uit 📻",
                    ready.user.name
                );
                Ok(Data(app))
            })
        })
        .build();

    let intents = serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_VOICE_STATES;
    let mut client = serenity::ClientBuilder::new(&config.discord_token, intents)
        .framework(framework)
        .await
        .context("kon Discord-client niet aanmaken")?;

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("afsluiten...");
            shard_manager.shutdown_all().await;
        }
    });

    client
        .start_autosharded()
        .await
        .context("Discord-client gestopt met een fout")?;
    Ok(())
}
