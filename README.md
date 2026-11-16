# Broodkast 🍞📻

**Broodkast** is de vernederlandsing van *broadcast*: een Nederlandstalige Discord-bot die uitzendt wat jij volgt. Nieuwe video's, blogposts, Twitch-streams en berichten uit je eigen tools komen vers uit de broodkast in je kanaal. En er is een koe die praat.

## Commando's

**Integraties** (alleen voor leden met *Server beheren*)

| Commando | Wat het doet |
|---|---|
| `/volg rss <url>` | Volg een RSS-, Atom- of JSON-feed |
| `/volg twitch <streamer>` | Melding wanneer een streamer live gaat |
| `/volg webhook <naam>` | Krijg een geheime URL waarmee andere diensten berichten posten |
| `/ontvolg <abonnement>` | Stop met een abonnement (met autocomplete) |
| `/volglijst` | Toon alles wat deze server volgt |

Bij elk `/volg`-commando kun je een ander `kanaal` kiezen en een `rol` die gepingd wordt.

**Leuk** (werken ook als je Broodkast voor jezelf installeert, in DM's en groepsgesprekken)

| Commando | Wat het doet |
|---|---|
| `/muntgooier` | Kop of munt |
| `/koepraat <tekst>` | Laat een koe je tekst zeggen |
| `/apod` | NASA's astronomiefoto van de dag |
| `/over` | Info over Broodkast en een uitnodigingslink |

### Handige feed-URL's

| Bron | Feed-URL |
|---|---|
| YouTube-kanaal | `https://www.youtube.com/feeds/videos.xml?channel_id=<kanaal-ID>` |
| Subreddit | `https://www.reddit.com/r/<naam>/new/.rss` |
| GitHub-releases | `https://github.com/<eigenaar>/<repo>/releases.atom` |

### Webhooks

```bash
curl -X POST https://<jouw-bot>/webhook/<token> \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy klaar", "message": "Versie 1.2 staat online", "url": "https://example.com"}'
```

Velden: `message` (of `content`), `title`, `url`, `image`, `color` (getal, bv. `16711680`). Een body die geen JSON is, wordt als platte tekst gepost. Maximaal 1 bericht per 2 seconden per webhook.

## Zelf draaien

1. Installeer [Rust](https://rustup.rs).
2. Kopieer `.env.example` naar `.env` en vul minstens `DISCORD_TOKEN` in.
3. Start de bot:

   ```bash
   cargo run
   ```

Met `DEV_GUILD_ID` ingevuld verschijnen de commando's meteen in je testserver; zonder worden ze globaal geregistreerd.

### Discord Developer Portal

- **Installation → Installation Contexts:** zet zowel *Guild Install* als *User Install* aan.
- **Guild Install scopes:** `bot` en `applications.commands`, met de rechten *Kanalen bekijken*, *Berichten verzenden* en *Links insluiten*.
- **User Install scopes:** `applications.commands`.

## Hosten op Railway

De repo bevat een `Dockerfile` en `railway.toml`; Railway bouwt daarmee automatisch.

1. **Volume:** voeg een volume toe aan de service, gemount op `/data`. Daar staat de SQLite-database; zonder volume ben je alle abonnementen kwijt bij elke deploy.
2. **Variabelen:** `DISCORD_TOKEN`, en optioneel `NASA_API_KEY`, `TWITCH_CLIENT_ID` en `TWITCH_CLIENT_SECRET`. Zet `DEV_GUILD_ID` niet in productie.
3. **Netwerk:** genereer een publiek domein (Settings → Networking). Dat is nodig voor webhooks; de bot leest het automatisch uit `RAILWAY_PUBLIC_DOMAIN`.

Draai maar één instantie tegelijk.

## Projectstructuur

- `bot/`: de Discord-bot (Rust, [poise](https://github.com/serenity-rs/poise) + serenity, SQLite via sqlx, axum)
- `legacy/`: de oude JavaScript-versie, alleen ter referentie
