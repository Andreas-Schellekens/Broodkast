use crate::{Context, Error};

/// Draai een muntje om
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn muntgooier(ctx: Context<'_>) -> Result<(), Error> {
    let uitkomst = if rand::random::<bool>() {
        "🪙 Kop!"
    } else {
        "🪙 Munt!"
    };
    ctx.say(uitkomst).await?;
    Ok(())
}
