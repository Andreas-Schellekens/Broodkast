use anyhow::{Context as _, bail};
use poise::serenity_prelude::GuildId;

/// Configuratie uit omgevingsvariabelen (of `.env` tijdens ontwikkeling).
pub struct Config {
    pub discord_token: String,
    /// Als dit gezet is, worden commando's alleen in deze server geregistreerd (direct zichtbaar).
    pub dev_guild_id: Option<GuildId>,
    pub nasa_api_key: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let discord_token = std::env::var("DISCORD_TOKEN")
            .context("DISCORD_TOKEN ontbreekt — zet die in .env (zie .env.example)")?;
        if discord_token.trim().is_empty() {
            bail!("DISCORD_TOKEN is leeg");
        }

        let dev_guild_id = match std::env::var("DEV_GUILD_ID") {
            Ok(v) if !v.trim().is_empty() => Some(GuildId::new(
                v.trim()
                    .parse()
                    .context("DEV_GUILD_ID moet een getal zijn")?,
            )),
            _ => None,
        };

        let nasa_api_key = std::env::var("NASA_API_KEY").unwrap_or_else(|_| "DEMO_KEY".to_owned());

        Ok(Self {
            discord_token,
            dev_guild_id,
            nasa_api_key,
        })
    }
}
