mod commands;
mod config;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

use crate::config::Config;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Gedeelde staat die elk commando via `ctx.data()` kan gebruiken.
pub struct Data {
    pub http: reqwest::Client,
    pub nasa_api_key: String,
}

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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                match config.dev_guild_id {
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
                Ok(Data {
                    http: reqwest::Client::builder()
                        .user_agent(concat!("Broodkast/", env!("CARGO_PKG_VERSION")))
                        .build()?,
                    nasa_api_key: config.nasa_api_key,
                })
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
