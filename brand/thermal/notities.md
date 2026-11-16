# Thermal

**Idee.** De broodkast door een warmtecamera: vers brood is warm, dus de kast gloeit. De homepage leent de taal van een
warmtecamera: een temperatuurschaal, een richtkruis met "212,4 °C · korst" en grote condensed cijfers. Daarbij een zwarte
grond, één korrelige hittegloed en harde kapitalen.

Canvas: https://claude.ai/artifact/DswmBgp8N23AqyfjNZcAeq (rij "Thermal"; bron in `brand/canvas/Thermal*.dc.html`).

## Palet
| Rol | Naam | Hex | Gebruik |
|---|---|---|---|
| main | Gloei | `#FF6A2B` | koppen van commando's, cijfers, logo, embedkleur van de bot |
| secondary | Roet | `#0B0806` | achtergrond |
| accent | Infrarood | `#FF2E88` | **call to action**: knoppen met donkere tekst en een roze gloed; ook live-stippen en de bovenplank van het logo |

De call to action moet knallen. Daarom is elke "Voeg toe aan Discord"-knop Infrarood met donkere tekst (5,6:1) en een
gloed (`--cta-glow`). De grote knoppen pulseren zacht, behalve bij `prefers-reduced-motion`. De knoppen staan altijd op
zwart: de hittegloed wordt aan de tekstkant weggewerkt met een zwarte overgang, zodat roze nooit op oranje staat.
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
