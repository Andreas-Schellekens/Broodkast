# Fluted glass

**Idee.** Een bakkersetalage achter geribbeld glas: warm ovenlicht dat in verticale banen breekt, met pruim in de schaduw.
De lamellen van het roldeksel in het logo zijn de ribbels. Rustig en sfeervol: veel donker en één warme gloed.

Canvas: https://claude.ai/artifact/DswmBgp8N23AqyfjNZcAeq (rij "Fluted glass"; bron in `site/drafts/canvas/Fluted*.dc.html`).

## Palet
| Rol | Naam | Hex | Gebruik |
|---|---|---|---|
| main | Gloed | `#EE8A3E` | knoppen, cijfers, het licht |
| secondary | Roet | `#120A08` | achtergrond |
| accent | Pruim | `#6B2457` | volle vlakken (commando's), handvat in het logo |
| neutraal | Kaars | `#F5E7DC` | tekst |
| neutraal | Rook | `#A8948A` | secundaire tekst |
| neutraal | Diep | `#0A0605` | footer |

## Typografie (Google Fonts)
- Display: **Syne** 700–800, krap gespatieerd.
- Tekst: **Hanken Grotesk** 400–600. Geen monospace.

## Beeld
`fluted.webp` is gemaakt met `fluted.py` (numpy + Pillow): een zachte achtergrond (bruin, een oranje band, pruim) die per ribbel
vergroot en verschoven wordt, met schaduw en een dunne lichtlijn op elke rand. Het ribbelpatroon komt als CSS-token
(`--ribbel`) terug in de kaarten en de footer. De "glazen" kaartjes gebruiken `backdrop-filter`.

## Embedkleur
`0xEE8A3E` (Gloed).
