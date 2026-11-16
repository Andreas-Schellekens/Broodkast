use std::{ops::Deref, sync::Arc};

use poise::serenity_prelude as serenity;
use sqlx::SqlitePool;

use crate::integrations::twitch::TwitchClient;

/// Alles wat commando's, achtergrondtaken en de webserver delen.
pub struct App {
    pub db: SqlitePool,
    /// HTTP-client voor externe API's en feeds.
    pub http: reqwest::Client,
    /// Discord REST-client; werkt los van de gateway-verbinding.
    pub discord: Arc<serenity::Http>,
    pub twitch: Option<TwitchClient>,
    pub nasa_api_key: String,
    pub public_url: Option<String>,
}

/// De poise user data. Een dunne wrapper zodat achtergrondtaken dezelfde `Arc<App>` krijgen.
pub struct Data(pub Arc<App>);

impl Deref for Data {
    type Target = App;
    fn deref(&self) -> &App {
        &self.0
    }
}
