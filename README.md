# Broodkast 🍞📻

**Broodkast** is de vernederlandsing van *broadcast*: een Nederlandstalige Discord-bot met vooral leuke en gekke functies. Oorspronkelijk een JavaScript-leerproject, nu herbouwd in Rust.

## Commando's

| Commando | Wat het doet |
|---|---|
| `/muntgooier` | Draait een muntje om: kop of munt |
| `/koepraat <tekst>` | Laat een koe je tekst zeggen |
| `/apod` | NASA's astronomiefoto van de dag |

Muziek komt in een latere fase terug.

## Zelf draaien

1. Installeer [Rust](https://rustup.rs).
2. Kopieer `.env.example` naar `.env` en vul `DISCORD_TOKEN` in.
3. Start de bot:

   ```bash
   cargo run
   ```

Met `DEV_GUILD_ID` ingevuld verschijnen de commando's meteen in je testserver; zonder worden ze globaal geregistreerd.

## Uitnodigen

Link komt hier, samen met de site.

## Projectstructuur

- `bot/`: de Discord-bot (Rust, [poise](https://github.com/serenity-rs/poise) + serenity)
- `legacy/`: de oude JavaScript-versie, alleen ter referentie
