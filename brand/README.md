# Broodkast: branding

Drie ontwerprichtingen voor de site op `broodkast.schellekensdigital.be`. Er is nog niet gekozen.
Elke map bevat `notities.md` (palet, fonts, idee) en `logo.svg` (schets van beeldmerk + woordmerk).
De bijbehorende homepage-drafts staan in `site/drafts/`.

| Richting | Sfeer | Main | Secondary | Accent | Fonts |
|---|---|---|---|---|---|
| 1. Nachtzender | donker, rustig, dicht bij Tastelabs | Ovengloed `#E9A23B` | Nachtblauw `#141831` | Ether `#9DB4FF` | Bricolage Grotesque + Albert Sans |
| 2. Omroepgids | licht, grafisch, zendergids | Gidsgroen `#1E5B47` | Korstbruin `#C8873A` | Signaalgeel `#F4C430` | Archivo (Expanded) + Instrument Sans |
| 3. Kruimel | speels, stickers, dikke lijnen | Toastoranje `#F27A2E` | Indigo `#2D2A6E` | Munt `#7EDDB5` | Bagel Fat One + Onest |

Vaste regels voor alle richtingen:
- Precies drie merkkleuren (main, secondary, accent); daarnaast alleen neutrale tinten voor tekst en achtergrond.
- Geen monospace, ook niet voor commando's of technische details. Commando's als `/volg rss` krijgen het gewone tekstfont in een "chip".

## Embedkleur voor de bot (voor de backend-sessie)

De embeds gebruiken nu `BROODKAST_KLEUR = 0xC8873A`. Per richting:
- Nachtzender: `0xE9A23B` (Ovengloed). Leest goed op Discords donkere en lichte thema.
- Omroepgids: `0x1E5B47` (Gidsgroen) als merk, of `0xC8873A` laten staan (Korstbruin is hier de secondary).
- Kruimel: `0xF27A2E` (Toastoranje).

Zodra er een richting gekozen is, staat de definitieve waarde hier en kan `bot/src/integrations/mod.rs` hem overnemen.

## Drafts (privé Artifacts)
- Nachtzender: https://claude.ai/artifact/RyRH37GYzMPuKoPdSc1zek (`site/drafts/1-nachtzender.html`)
- Omroepgids: https://claude.ai/artifact/7aDzcqccj1XuwFskPLWnib (`site/drafts/2-omroepgids.html`)
- Kruimel: https://claude.ai/artifact/WFbSGbYj7i7LbvRRfwuzkD (`site/drafts/3-kruimel.html`)

De drafts zijn losse HTML-bestanden zonder build. De Google Fonts worden via een `<link>` geladen.

## Naar SvelteKit

De echte site wordt SvelteKit met `adapter-cloudflare` in `site/`. Die wordt pas gebouwd als er een richting gekozen is.

**Tokens.** Elke richting heeft een `tokens.css` met `--main`, `--secondary` en `--accent`, plus neutrale tokens (`--bg`, `--surface`, `--text`, `--text-muted`, `--line`), font-tokens en radii. Bovenaan staat de Google Fonts-URL. In SvelteKit importeer je dat bestand één keer in `+layout.svelte` en zet je de font-`<link>` in `app.html`. De drafts gebruiken dezelfde variabelenamen voor de merkkleuren.

**Blokken die componenten worden.** Deze blokken komen in alle drie de drafts voor:

| Component | In de drafts | Props |
|---|---|---|
| `SiteNav` | logo, ankerlinks, knop | links, inviteUrl |
| `InviteButton` | "Voeg toe aan Discord" | variant (main/rand), label |
| `Hero` | kop, tekst, knoppen + beeldmoment (waaier, gids, figuurtje) | slot voor het beeld |
| `NotificationCard` | sneetje / gidsregel / sticker met bron, titel, kanaal | bron, titel, meta, live |
| `FeatureCard` | feeds, Twitch, webhooks | titel, tekst, tags, commando, ritme |
| `CommandChip` | `/volg rss` in het tekstfont (geen mono) | naam |
| `CommandDemo` | /koepraat, /muntgooier, /apod | naam, uitleg, slot voor demo |
| `SiteFooter` | merk, twee linkkolommen, onderregel | links |

De invite-URL is `https://discord.com/oauth2/authorize?client_id=1377039869645295656`. Zet die in één config-bestand.

**Ruimte voor het dashboard.** Later komt er een deel waar je met Discord inlogt en je abonnementen beheert. Elke richting heeft daar al een `--surface`-token voor (panelen, tabellen, formulieren). Per richting:
- Nachtzender: de app blijft donker op `--surface`. Ovengloed alleen voor de primaire actie, Ether voor de status (live, actief).
- Omroepgids: de app is licht op `--surface` (wit). Gidsgroen voor de navigatiebalk en de soort-labels, geel alleen voor de primaire actie. Een abonnementenlijst kan het "schema" van de hero worden: een rij per bron, met de laatste melding en het tijdstip.
- Kruimel: in de app dunnere lijnen (2px) en geen gekantelde kaartjes, anders wordt hij onrustig. De harde schaduw is alleen voor knoppen.
