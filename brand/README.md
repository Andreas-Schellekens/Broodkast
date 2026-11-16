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
