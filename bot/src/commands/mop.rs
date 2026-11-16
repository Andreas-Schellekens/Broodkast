use crate::{Context, Error};

/// (vraag, clou). De clou staat achter een spoiler, zodat je eerst zelf kunt raden.
const MOPPEN: &[(&str, &str)] = &[
    (
        "Waarom ging het koekje naar de dokter?",
        "Het voelde zich een beetje kruimelig.",
    ),
    (
        "Wat is zwart-wit en zit in een boom?",
        "Een koe die de weg kwijt is.",
    ),
    (
        "Wat doet een koe als ze een goeie mop hoort?",
        "Ze loeit van het lachen.",
    ),
    ("Hoe noem je een boemerang die niet terugkomt?", "Een stok."),
    ("Wat zegt de nul tegen de acht?", "Mooie riem!"),
    (
        "Waarom doen skeletten nooit mee aan wedstrijden?",
        "Ze hebben er het hart niet voor.",
    ),
    (
        "Waarom zijn olifanten zo slecht met computers?",
        "Ze zijn bang van de muis.",
    ),
    (
        "Wat zegt een slak op de rug van een schildpad?",
        "Joehoe, wat gaat dat snel!",
    ),
    (
        "Wat doet een programmeur als hij het koud heeft?",
        "Hij gaat in de hoek zitten, daar is het 90 graden.",
    ),
    (
        "Hoeveel soorten mensen zijn er?",
        "10: zij die binair snappen en zij die dat niet doen.",
    ),
    (
        "Wat zit er in een broodkast en zendt uit?",
        "Broodkast natuurlijk. 📻",
    ),
    (
        "Ken je die mop over het brood?",
        "Laat maar, die is een beetje oudbakken.",
    ),
    (
        "Wat is het toppunt van luiheid?",
        "Je wekker op snooze zetten voordat hij afgaat.",
    ),
    (
        "Wat is het toppunt van eenzaamheid?",
        "Een paasei vinden dat je zelf verstopt hebt.",
    ),
    ("Wat is de lievelingsfilm van een boterham?", "Toast Story."),
    ("Wat zegt de ene muur tegen de andere?", "Tot op de hoek!"),
];

/// Vertel een mop
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn mop(ctx: Context<'_>) -> Result<(), Error> {
    let (vraag, clou) = MOPPEN[rand::random_range(0..MOPPEN.len())];
    ctx.say(format!("😄 {vraag}\n||{clou}||")).await?;
    Ok(())
}
