use crate::{Context, Error};

/// Draai een muntje om
#[poise::command(slash_command)]
pub async fn muntgooier(ctx: Context<'_>) -> Result<(), Error> {
    let uitkomst = if rand::random::<bool>() {
        "🪙 Kop!"
    } else {
        "🪙 Munt!"
    };
    ctx.say(uitkomst).await?;
    Ok(())
}
