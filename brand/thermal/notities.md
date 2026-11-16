# Thermal

**Idee.** De broodkast door een warmtecamera: vers brood is warm, dus de kast gloeit. De homepage leent de taal van een
warmtecamera: een temperatuurschaal, een richtkruis met "212,4 °C · korst" en grote condensed cijfers. Daarbij een zwarte
grond, één korrelige hittegloed en harde kapitalen.

Canvas: https://claude.ai/artifact/DswmBgp8N23AqyfjNZcAeq (rij "Thermal"; bron in `site/drafts/canvas/Thermal*.dc.html`).

## Palet
| Rol | Naam | Hex | Gebruik |
|---|---|---|---|
| main | Gloei | `#FF6A2B` | knoppen, koppen van commando's |
| secondary | Roet | `#0B0806` | achtergrond |
| accent | Infrarood | `#FF2E88` | stippen, live, bovenplank van het logo |
| neutraal | Wit | `#FFFFFF` | tekst |
| neutraal | As | `#B3ABA5` | secundaire tekst |
| neutraal | Kool | `#17120F` | panelen |

De warmteschaal (blauw, violet, roze, oranje, geel) is beeld: het hero-beeld, de HUD-balk en de verlooptekst. Gebruik
haar niet voor tekst of knoppen.

## Typografie (Google Fonts)
- Display: **Big Shoulders Display** 800–900, in kapitalen. Ook voor temperaturen en cijfers (`tabular-nums`).
- Tekst: **Inter Tight** 400–600. Geen monospace.

## Beeld
`thermal.webp` is gemaakt met `thermal.py` (numpy + Pillow): een gedraaide elliptische gradiënt in de warmteschaal, met filmkorrel.

## Embedkleur
`0xFF6A2B` (Gloei).
