//! `/weer`: het weer van nu en de komende dagen, via Open-Meteo (gratis, geen sleutel nodig).

use anyhow::Context as _;
use chrono::{Datelike, NaiveDate, Weekday};
use poise::serenity_prelude as serenity;
use serde::Deserialize;

use crate::{Context, Error, integrations::BROODKAST_KLEUR};

const GEO_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const WEER_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Deserialize)]
struct GeoAntwoord {
    results: Option<Vec<Plaats>>,
}

#[derive(Deserialize)]
struct Plaats {
    name: String,
    latitude: f64,
    longitude: f64,
    country: Option<String>,
    admin1: Option<String>,
}

#[derive(Deserialize)]
struct Voorspelling {
    current: Huidig,
    daily: Dagelijks,
}

#[derive(Deserialize)]
struct Huidig {
    temperature_2m: f64,
    apparent_temperature: f64,
    relative_humidity_2m: f64,
    weather_code: u8,
    wind_speed_10m: f64,
}

#[derive(Deserialize)]
struct Dagelijks {
    time: Vec<String>,
    weather_code: Vec<Option<u8>>,
    temperature_2m_max: Vec<Option<f64>>,
    temperature_2m_min: Vec<Option<f64>>,
    precipitation_probability_max: Vec<Option<f64>>,
}

/// Het weer van nu en de komende dagen
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn weer(
    ctx: Context<'_>,
    #[description = "Plaats, bv. Antwerpen of Utrecht"]
    #[max_length = 100]
    plaats: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let http = &ctx.data().http;

    let geo: GeoAntwoord = http
        .get(GEO_URL)
        .query(&[
            ("name", plaats.trim()),
            ("count", "1"),
            ("language", "nl"),
            ("format", "json"),
        ])
        .send()
        .await
        .context("weerdienst niet bereikbaar")?
        .error_for_status()?
        .json()
        .await
        .context("weerdienst gaf onverwachte data")?;
    let Some(plek) = geo.results.and_then(|r| r.into_iter().next()) else {
        ctx.say(format!(
            "❌ Ik vind geen plaats met de naam **{}**.",
            plaats.trim()
        ))
        .await?;
        return Ok(());
    };

    let v: Voorspelling = http
        .get(WEER_URL)
        .query(&[
            ("latitude", plek.latitude.to_string()),
            ("longitude", plek.longitude.to_string()),
            (
                "current",
                "temperature_2m,apparent_temperature,relative_humidity_2m,weather_code,wind_speed_10m"
                    .into(),
            ),
            (
                "daily",
                "weather_code,temperature_2m_max,temperature_2m_min,precipitation_probability_max"
                    .into(),
            ),
            ("timezone", "auto".into()),
            ("forecast_days", "4".into()),
        ])
        .send()
        .await
        .context("weerdienst niet bereikbaar")?
        .error_for_status()?
        .json()
        .await
        .context("weerdienst gaf onverwachte data")?;

    let locatie = [Some(plek.name), plek.admin1, plek.country]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ");
    let (emoji, omschrijving) = weertype(v.current.weather_code);
    let c = &v.current;

    let mut embed = serenity::CreateEmbed::new()
        .title(format!("{emoji} Weer in {locatie}"))
        .description(format!(
            "**{}°C** · {omschrijving}\nVoelt als {}°C · 💨 {:.0} km/u · 💧 {:.0}%",
            graden(c.temperature_2m),
            graden(c.apparent_temperature),
            c.wind_speed_10m,
            c.relative_humidity_2m,
        ))
        .footer(serenity::CreateEmbedFooter::new(
            "Weergegevens: Open-Meteo.com",
        ))
        .colour(BROODKAST_KLEUR);

    let d = &v.daily;
    for (i, datum) in d.time.iter().enumerate() {
        let naam = if i == 0 {
            "Vandaag".to_owned()
        } else {
            NaiveDate::parse_from_str(datum, "%Y-%m-%d")
                .map(|n| dagnaam(n.weekday()).to_owned())
                .unwrap_or_else(|_| datum.clone())
        };
        let code = d.weather_code.get(i).copied().flatten();
        let min = d.temperature_2m_min.get(i).copied().flatten();
        let max = d.temperature_2m_max.get(i).copied().flatten();
        let regen = d.precipitation_probability_max.get(i).copied().flatten();
        let mut waarde = format!(
            "{} {}° / {}°",
            code.map(|c| weertype(c).0).unwrap_or("❔"),
            min.map(graden).unwrap_or_else(|| "?".into()),
            max.map(graden).unwrap_or_else(|| "?".into()),
        );
        if let Some(p) = regen {
            waarde.push_str(&format!("\n☔ {p:.0}%"));
        }
        embed = embed.field(naam, waarde, true);
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Afgerond op hele graden, zonder "-0".
fn graden(t: f64) -> String {
    let r = t.round();
    if r == 0.0 {
        "0".into()
    } else {
        format!("{r:.0}")
    }
}

fn dagnaam(dag: Weekday) -> &'static str {
    match dag {
        Weekday::Mon => "Maandag",
        Weekday::Tue => "Dinsdag",
        Weekday::Wed => "Woensdag",
        Weekday::Thu => "Donderdag",
        Weekday::Fri => "Vrijdag",
        Weekday::Sat => "Zaterdag",
        Weekday::Sun => "Zondag",
    }
}

/// WMO-weercode naar emoji en Nederlandse omschrijving.
fn weertype(code: u8) -> (&'static str, &'static str) {
    match code {
        0 => ("☀️", "Helder"),
        1 => ("🌤️", "Overwegend helder"),
        2 => ("⛅", "Half bewolkt"),
        3 => ("☁️", "Bewolkt"),
        45 | 48 => ("🌫️", "Mist"),
        51 | 53 | 55 => ("🌦️", "Motregen"),
        56 | 57 | 66 | 67 => ("🧊", "IJzel"),
        61 => ("🌧️", "Lichte regen"),
        63 => ("🌧️", "Regen"),
        65 => ("🌧️", "Zware regen"),
        71 | 73 | 75 | 77 => ("❄️", "Sneeuw"),
        80..=82 => ("🌦️", "Regenbuien"),
        85 | 86 => ("🌨️", "Sneeuwbuien"),
        95 => ("⛈️", "Onweer"),
        96 | 99 => ("⛈️", "Onweer met hagel"),
        _ => ("🌡️", "Onbekend"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graden_zonder_min_nul() {
        assert_eq!(graden(-0.3), "0");
        assert_eq!(graden(12.6), "13");
        assert_eq!(graden(-4.5), "-5");
    }
}
