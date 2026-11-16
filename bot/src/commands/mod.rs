//! Alle slash-commando's. Voeg een nieuw commando toe als eigen module en zet het in [`all`].
//!
//! Commando's die overal mogen werken (ook als de gebruiker de bot zelf installeert, in DM's)
//! krijgen `install_context = "Guild|User"` en `interaction_context = "Guild|BotDm|PrivateChannel"`.
//! Commando's die serverinstellingen wijzigen blijven `Guild`-only.

mod apod;
mod koepraat;
mod muntgooier;
mod over;
mod volg;

use crate::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        // Leuk
        muntgooier::muntgooier(),
        koepraat::koepraat(),
        apod::apod(),
        over::over(),
        // Integraties
        volg::volg(),
        volg::ontvolg(),
        volg::volglijst(),
    ]
}
