use poise::serenity_prelude as serenity;

use crate::{Context, Error, util::truncate};

const ANTWOORDEN: &[&str] = &[
    "Zeker weten.",
    "Zonder twijfel.",
    "Ja, absoluut.",
    "Reken er maar op.",
    "Zoals ik het zie: ja.",
    "Waarschijnlijk wel.",
    "Ziet er goed uit.",
    "De tekenen wijzen op ja.",
    "Vraag het later nog eens.",
    "Dat kan ik nu beter niet zeggen.",
    "Het brood is nog niet gaar. Vraag het straks opnieuw.",
    "Concentreer je en vraag het opnieuw.",
    "Reken er niet op.",
    "Mijn antwoord is nee.",
    "Mijn bronnen zeggen nee.",
    "Ziet er niet zo goed uit.",
    "Zeer twijfelachtig.",
    "Absoluut niet.",
];

/// Stel een ja-neevraag aan de glazen bol
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn glazenbol(
    ctx: Context<'_>,
    #[description = "Je vraag"]
    #[max_length = 200]
    vraag: String,
) -> Result<(), Error> {
    let antwoord = ANTWOORDEN[rand::random_range(0..ANTWOORDEN.len())];
    ctx.send(
        poise::CreateReply::default()
            .content(format!("> {}\n🔮 {antwoord}", truncate(vraag.trim(), 200)))
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}
