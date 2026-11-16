use poise::serenity_prelude as serenity;

use crate::{Context, Error, util::truncate};

/// Laat Broodkast kiezen, bv. "pizza, frietjes of sushi"
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn kies(
    ctx: Context<'_>,
    #[description = "Opties, gescheiden door komma's of \"of\""]
    #[max_length = 500]
    opties: String,
) -> Result<(), Error> {
    let lijst = splits(&opties);
    let tekst = if lijst.len() < 2 {
        "🤔 Geef me minstens twee opties, bv. `pizza, frietjes of sushi`.".to_owned()
    } else {
        let keuze = lijst[rand::random_range(0..lijst.len())];
        format!("🤔 Ik kies... **{}**!", truncate(keuze, 200))
    };
    ctx.send(
        poise::CreateReply::default()
            .content(tekst)
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}

fn splits(opties: &str) -> Vec<&str> {
    opties
        .split([',', '|', ';'])
        .flat_map(|deel| deel.split(" of "))
        .map(str::trim)
        .filter(|o| !o.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitst_opties() {
        assert_eq!(
            splits("pizza, frietjes of sushi"),
            ["pizza", "frietjes", "sushi"]
        );
        assert_eq!(splits("ja|nee"), ["ja", "nee"]);
        assert_eq!(splits("  , alleen dit ,"), ["alleen dit"]);
        // "of" binnen een woord splitst niet.
        assert_eq!(splits("koffie of thee"), ["koffie", "thee"]);
        assert_eq!(splits("sofa"), ["sofa"]);
    }
}
