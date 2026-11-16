# Broodkast: branding

Drie ontwerprichtingen voor `broodkast.schellekensdigital.be`, alle drie op één Claude Design-canvas:
https://claude.ai/artifact/DswmBgp8N23AqyfjNZcAeq (privé). Elke richting heeft een homepage en een merkblad.
**Gekozen: Thermal** (16-11-2026). De eerdere richtingen (Nachtzender, Omroepgids, Kruimel) zijn op verzoek geschrapt.

| Richting | Sfeer | Main | Secondary | Accent | Fonts |
|---|---|---|---|---|---|
| Stippel | strak, nachtblauw, stippel-gradiënt | Signaal `#5B5CF0` | Nachtinkt `#030B3F` | Beukenhout `#D9913F` | Sora + Inter |
| Fluted glass | etalage achter geribbeld glas | Gloed `#EE8A3E` | Roet `#120A08` | Pruim `#6B2457` | Syne + Hanken Grotesk |
| Thermal | warmtecamera, korrelige hittegloed | Gloei `#FF6A2B` | Roet `#0B0806` | Infrarood `#FF2E88` | Big Shoulders Display + Inter Tight |

Alle drie gebruiken hetzelfde beeldmerk: een broodkast met roldeksel en een kleine antenne. Alleen de kleuren verschillen.

Per map vind je:
- `notities.md`: idee, palet en fonts;
- `tokens.css`: CSS-variabelen en de Google Fonts-URL;
- `logo.svg`;
- het achtergrondbeeld (`.webp`) en het Python-script dat het maakt.

De bron van de canvas staat in `brand/canvas/` (de `.dc.html`-artboards en `canvas.json`).

Vaste regels:
- Precies drie merkkleuren (main, secondary, accent); daarnaast alleen neutrale tinten.
- Geen monospace, ook niet voor commando's.

## Embedkleur voor de bot (voor de backend-sessie)
Gekozen: Thermal, dus `BROODKAST_KLEUR = 0xFF6A2B` (Gloei). Ter info, de andere waarden waren: Stippel `0x5B5CF0`, Fluted glass `0xEE8A3E`, Thermal `0xFF6A2B`.

## Naar SvelteKit
De echte site staat in `site/` (SvelteKit + `adapter-cloudflare`), gebouwd in de Thermal-stijl.
`tokens.css` gaat één keer in `+layout.svelte`, de font-`<link>` in `app.html`.

Deze blokken komen in alle drie de homepages terug en worden componenten:
- `SiteNav`
- `InviteButton`
- `Hero`, met een slot voor het beeld
- `NotificationCard`
- `FeatureCard`: titel, frequentie, tekst, commando en limiet
- `CommandChip`: in het tekstfont, geen mono
- `CommandDemo`
- `SiteFooter`

De invite-URL `https://discord.com/oauth2/authorize?client_id=1377039869645295656` komt in één config-bestand.

Later komt er een dashboard waar je met Discord inlogt en je abonnementen beheert. Elke richting heeft daarvoor een
`--surface`-token voor panelen en tabellen. Het beeld (stippel, ribbelglas, hittegloed) hoort bij de landingspagina.
In de app komt het hooguit terug als smalle band in de kop.
