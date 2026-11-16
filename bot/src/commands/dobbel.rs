use crate::{Context, Error};

const MAX_STENEN: u32 = 20;
const MAX_ZIJDEN: u32 = 1000;

/// Gooi dobbelstenen, bv. 2d6 of d20+3
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn dobbel(
    ctx: Context<'_>,
    #[description = "Welke worp? bv. d6, 2d6 of d20+3 (standaard 1d6)"]
    #[max_length = 20]
    worp: Option<String>,
) -> Result<(), Error> {
    let notatie = worp.as_deref().unwrap_or("1d6");
    let Some(w) = Worp::parse(notatie) else {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "❌ Die worp snap ik niet. Probeer `d6`, `2d6` of `d20+3` \
                     (max {MAX_STENEN} stenen van {MAX_ZIJDEN} zijden)."
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let ogen: Vec<u32> = (0..w.aantal)
        .map(|_| rand::random_range(1..=w.zijden))
        .collect();
    let som: i64 = ogen.iter().map(|&o| o as i64).sum::<i64>() + w.bonus;

    let mut tekst = format!("🎲 **{}**: ", w.notatie());
    if ogen.len() == 1 && w.bonus == 0 {
        tekst.push_str(&format!("**{som}**"));
    } else {
        let lijst: Vec<String> = ogen.iter().map(u32::to_string).collect();
        tekst.push_str(&format!("[{}]", lijst.join(", ")));
        if w.bonus != 0 {
            tekst.push_str(&format!(
                " {} {}",
                if w.bonus > 0 { '+' } else { '-' },
                w.bonus.abs()
            ));
        }
        tekst.push_str(&format!(" = **{som}**"));
    }
    ctx.say(tekst).await?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Worp {
    aantal: u32,
    zijden: u32,
    bonus: i64,
}

impl Worp {
    /// Leest `NdM`, `dM`, `NdM+K` of `NdM-K` (ook met `w` in plaats van `d`).
    fn parse(input: &str) -> Option<Worp> {
        let tekst: String = input
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let (aantal, rest) = tekst.split_once(['d', 'w'])?;
        let aantal = if aantal.is_empty() {
            1
        } else {
            aantal.parse().ok()?
        };
        let (zijden, bonus) = match rest.find(['+', '-']) {
            Some(i) => (&rest[..i], rest[i..].parse::<i64>().ok()?),
            None => (rest, 0),
        };
        let zijden = zijden.parse().ok()?;
        let geldig = (1..=MAX_STENEN).contains(&aantal)
            && (2..=MAX_ZIJDEN).contains(&zijden)
            && bonus.abs() <= 1000;
        geldig.then_some(Worp {
            aantal,
            zijden,
            bonus,
        })
    }

    fn notatie(&self) -> String {
        match self.bonus {
            0 => format!("{}d{}", self.aantal, self.zijden),
            b if b > 0 => format!("{}d{}+{b}", self.aantal, self.zijden),
            b => format!("{}d{}{b}", self.aantal, self.zijden),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leest_worpen() {
        let w = |aantal, zijden, bonus| {
            Some(Worp {
                aantal,
                zijden,
                bonus,
            })
        };
        assert_eq!(Worp::parse("d6"), w(1, 6, 0));
        assert_eq!(Worp::parse("2d6"), w(2, 6, 0));
        assert_eq!(Worp::parse("d20+3"), w(1, 20, 3));
        assert_eq!(Worp::parse("3D8 - 2"), w(3, 8, -2));
        assert_eq!(Worp::parse("2w10"), w(2, 10, 0));
    }

    #[test]
    fn weigert_onzin() {
        assert_eq!(Worp::parse("6"), None);
        assert_eq!(Worp::parse("d1"), None);
        assert_eq!(Worp::parse("0d6"), None);
        assert_eq!(Worp::parse("21d6"), None);
        assert_eq!(Worp::parse("d6+abc"), None);
    }

    #[test]
    fn notatie() {
        assert_eq!(Worp::parse("d20+3").unwrap().notatie(), "1d20+3");
        assert_eq!(Worp::parse("2d6-1").unwrap().notatie(), "2d6-1");
    }
}
