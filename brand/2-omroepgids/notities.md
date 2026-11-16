# Richting 2: Omroepgids

**Idee.** Broodkast is een zender, en een zender heeft een gids. De site leent de taal van de oude omroepgids:
een netlogo, een programmaschema met tijden, "tussendoortjes" tussen de programma's, een colofon onderaan.
De tijden zijn echt: feeds worden elke 10 minuten bekeken, Twitch elke 2 minuten, webhooks komen meteen door.
Licht en grafisch, met grote brede letters en volle kleurvlakken in plaats van dunne lijntjes.

## Palet
| Rol | Naam | Hex | Gebruik |
|---|---|---|---|
| main | Gidsgroen | `#1E5B47` | grote vlakken, netlogo, koppen |
| secondary | Korstbruin | `#C8873A` | de huidige botkleur; tijdstippen, tweede vlak |
| accent | Signaalgeel | `#F4C430` | knop "Voeg toe aan Discord", markeringen |
| neutraal | Papier | `#ECEFE8` | achtergrond (licht groenig, geen crème) |
| neutraal | Inkt | `#17201C` | tekst |
| neutraal | Grafiet | `#56635C` | secundaire tekst |

## Typografie (Google Fonts)
- Display: **Archivo** met breedte-as op 125% (Expanded), gewicht 800. Breed, zelfverzekerd, zoals zenderlogo's.
- Tekst: **Instrument Sans** 400–600.
- Tijden en cijfers: Archivo met `tabular-nums`, geen monospace.

## Logo
`logo.svg`: een groene zenderbadge met "BK" in geel en een bruin stipje (zender aan). Woordmerk BROODKAST in Archivo Expanded,
"KAST" in Gidsgroen.

## Toon
Zakelijk-vrolijk, zoals een omroeper. "Vanavond op Broodkast: jouw feeds, live."
