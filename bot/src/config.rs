use anyhow::{Context as _, bail};
use poise::serenity_prelude::GuildId;

/// Configuratie uit omgevingsvariabelen (of `.env` tijdens ontwikkeling). Zie `.env.example`.
pub struct Config {
    pub discord_token: String,
    /// Als dit gezet is, worden commando's alleen in deze server geregistreerd (direct zichtbaar).
    pub dev_guild_id: Option<GuildId>,
    pub nasa_api_key: String,
    pub database_url: String,
    /// Poort voor de webserver (health check + inkomende webhooks). Railway zet `PORT` zelf.
    pub port: u16,
    /// Publieke basis-URL waarop de webserver bereikbaar is, zonder `/` op het einde.
    pub public_url: Option<String>,
    pub twitch: Option<TwitchConfig>,
}

pub struct TwitchConfig {
    pub client_id: String,
    pub client_secret: String,
}

fn var(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let Some(discord_token) = var("DISCORD_TOKEN") else {
            bail!("DISCORD_TOKEN ontbreekt — zet die in .env (zie .env.example)");
        };

        let dev_guild_id = var("DEV_GUILD_ID")
            .map(|v| v.parse().map(GuildId::new))
            .transpose()
            .context("DEV_GUILD_ID moet een getal zijn")?;

        let port = var("PORT")
            .map(|v| v.parse())
            .transpose()
            .context("PORT moet een getal zijn")?
            .unwrap_or(8080);

        // Railway geeft het publieke domein mee als RAILWAY_PUBLIC_DOMAIN.
        let public_url = var("PUBLIC_URL")
            .or_else(|| var("RAILWAY_PUBLIC_DOMAIN").map(|d| format!("https://{d}")))
            .map(|u| u.trim_end_matches('/').to_owned());

        let twitch = match (var("TWITCH_CLIENT_ID"), var("TWITCH_CLIENT_SECRET")) {
            (Some(client_id), Some(client_secret)) => Some(TwitchConfig {
                client_id,
                client_secret,
            }),
            (None, None) => None,
            _ => bail!("zet zowel TWITCH_CLIENT_ID als TWITCH_CLIENT_SECRET, of geen van beide"),
        };

        Ok(Self {
            discord_token,
            dev_guild_id,
            nasa_api_key: var("NASA_API_KEY").unwrap_or_else(|| "DEMO_KEY".to_owned()),
            database_url: var("DATABASE_URL").unwrap_or_else(|| "sqlite://broodkast.db".to_owned()),
            port,
            public_url,
            twitch,
        })
    }
}
