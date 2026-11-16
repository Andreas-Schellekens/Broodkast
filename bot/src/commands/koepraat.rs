use crate::{Context, Error};

const MAX_LENGTE: usize = 50;

/// Praat als een koe
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn koepraat(
    ctx: Context<'_>,
    #[description = "Wat moet onze lieve koe zeggen"] tekst: String,
) -> Result<(), Error> {
    let tekst = tekst.trim();
    if tekst.chars().count() > MAX_LENGTE {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "Das te lang, womp womp. Minder dan {MAX_LENGTE} karakters aub."
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    ctx.say(format!("```\n{}\n```", koe(tekst))).await?;
    Ok(())
}

/// Tekent een koe met een tekstballon, zoals het klassieke `cowsay`.
fn koe(tekst: &str) -> String {
    // Backticks zouden het codeblok in Discord breken.
    let tekst = tekst.replace('`', "'");
    let breedte = tekst.chars().count();
    let rand = |c: char| c.to_string().repeat(breedte + 2);
    format!(
        " {boven}\n< {tekst} >\n {onder}\n{koe}",
        boven = rand('_'),
        onder = rand('-'),
        koe = r"        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||"
    )
}

#[cfg(test)]
mod tests {
    use super::koe;

    #[test]
    fn ballon_past_rond_tekst() {
        let uit = koe("boe");
        let mut regels = uit.lines();
        assert_eq!(regels.next(), Some(" _____"));
        assert_eq!(regels.next(), Some("< boe >"));
        assert_eq!(regels.next(), Some(" -----"));
    }

    #[test]
    fn backticks_worden_vervangen() {
        assert!(!koe("a```b").contains('`'));
    }
}
