//! Alle slash-commando's. Voeg een nieuw commando toe als eigen module en zet het in [`all`].

mod apod;
mod koepraat;
mod muntgooier;

use crate::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![muntgooier::muntgooier(), koepraat::koepraat(), apod::apod()]
}
