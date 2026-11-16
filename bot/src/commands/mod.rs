//! Alle slash-commando's. Voeg een nieuw commando toe als eigen module en zet het in [`all`].
//!
//! Commando's die overal mogen werken (ook als de gebruiker de bot zelf installeert, in DM's)
//! krijgen `install_context = "Guild|User"` en `interaction_context = "Guild|BotDm|PrivateChannel"`.
//! Commando's die serverinstellingen wijzigen blijven `Guild`-only.

mod apod;
mod dobbel;
mod gebruiker;
mod glazenbol;
pub mod herinner;
mod kies;
mod koepraat;
mod mop;
mod muntgooier;
mod opruimen;
mod over;
mod serverinfo;
mod volg;
mod weer;

use crate::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        // Leuk
        muntgooier::muntgooier(),
        dobbel::dobbel(),
        kies::kies(),
        glazenbol::glazenbol(),
        mop::mop(),
        koepraat::koepraat(),
        // Handig
        herinner::herinner(),
        weer::weer(),
        apod::apod(),
        gebruiker::gebruiker(),
        serverinfo::serverinfo(),
        opruimen::opruimen(),
        over::over(),
        // Integraties
        volg::volg(),
        volg::ontvolg(),
        volg::volglijst(),
    ]
}
