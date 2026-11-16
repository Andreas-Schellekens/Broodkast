# Richting 4: Stippel

**Idee.** Strakker dan de eerste drie, maar nog steeds met een knipoog. Het merk staat op een diepe nachtblauwe grond met een
gestippelde gradiënt die van leeg naar vol loopt, alsof een signaal steeds sterker binnenkomt. Het beeldmerk is een echte
broodkast met roldeksel in beukenhout, met een kleine antenne erop. Het warme hout tegen het koele blauw is het contrast
waar de hele richting op leunt. Layout: rustige grid, 1px-lijnen, kleine radii, één lichte sectie voor de integraties.

Gemaakt als canvas in Claude Design: https://claude.ai/artifact/DswmBgp8N23AqyfjNZcAeq
(artboards "Homepage" en "Merkblad"; bron in `site/drafts/4-stippel/`).

## Palet
| Rol | Naam | Hex | Gebruik |
|---|---|---|---|
| main | Signaal | `#5B5CF0` | knoppen, de stippels, het antennebolletje |
| secondary | Nachtinkt | `#030B3F` | achtergrond, tekst op licht |
| accent | Beukenhout | `#D9913F` | het logo, kleine signalen (het muntje, een stip) |
| neutraal | Wit / Mist | `#FFFFFF` / `#EEF0FA` | tekst op donker / lichte sectie |
| neutraal | Nevel | `#9AA0C8` | secundaire tekst op donker |
| neutraal | Diep | `#01061F` | footer, banden |

De stippels gebruiken tinten van Signaal (lichter, iets violetter, een enkele cyaan en witte stip). De houttinten
`#E6A65C`, `#B06D2A` en `#8E561F` zijn alleen voor de schaduwkanten van het logo.

## Typografie (Google Fonts)
- Display: **Sora** 600, strak gespatieerd (−0,035em).
- Tekst: **Inter** 400–600.
- Geen monospace, ook niet voor commando's.

## Logo
`logo.svg`: broodkast in driekwart-aanzicht met lamellen, handvat, bovenplank en zijpaneel, plus een antenne met
een Signaal-bolletje en een golfje. Op donker wordt de antenne `#E9EBF8`. In het merkblad staan ook een app-icoon
(Discord-avatar) en een vereenvoudigd favicon.

## Stippel
`stippel.webp` is gegenereerd met `stipple.py` (Pillow): een raster met jitter, waarbij de dichtheid en de stipgrootte naar
rechts toenemen. Opnieuw maken: `python stipple.py uit.png`. Bij SvelteKit kan dit ook live op een `<canvas>`.

## Embedkleur
`0x5B5CF0` (Signaal). Leest goed op Discords donkere en lichte thema.
