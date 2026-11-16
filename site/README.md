# Broodkast-site

Landings- en uitnodigingspagina voor de Broodkast-bot, in de Thermal-stijl (zie `../brand/thermal/`).
SvelteKit (Svelte 5) met `@sveltejs/adapter-cloudflare`, bedoeld voor `broodkast.schellekensdigital.be`.

```bash
npm install
npm run dev      # http://localhost:5173
npm run check    # svelte-check
npm run build    # bouwt voor Cloudflare naar .svelte-kit/cloudflare
```

## Opbouw
- `src/lib/styles/tokens.css`: kleuren, CTA-tokens, fonts en ritme. Precies drie merkkleuren: `--main` (Gloei),
  `--secondary` (Roet) en `--accent` (Infrarood, de call to action). Geen monospace.
- `src/lib/styles/global.css`: basis, `.wrap`, `.display`, `.hot` (verlooptekst), `.eyebrow`.
- `src/app.html`: Google Fonts (Big Shoulders Display, Inter Tight) en de favicon.
- `src/lib/config.ts`: de invite-URL en site-URL, op één plek.
- `src/lib/components/`: `SiteNav`, `Logo`, `InviteButton` (cta/ghost), `Hero` (snippets `kop` en `beeld`),
  `ThermalHud`, `NotificationCard`, `FeatureCard`, `CommandChip`, `CommandDemo`, `KoeDemo`, `MuntDemo`,
  `CtaBand`, `SiteFooter`.
- `src/routes/+page.svelte`: de homepage. `+layout.ts` zet `prerender = true`, dus de pagina is volledig statisch.
- `static/thermal.webp`: de hittegloed, gemaakt met `../brand/thermal/thermal.py`.

## Later
Het dashboard (inloggen met Discord, abonnementen beheren) kan als eigen routes onder bijvoorbeeld `/dashboard`
komen, met `prerender = false` en server-routes op Cloudflare. De panelen gebruiken `--surface`.

Deployen is nog niet ingericht. Dat kan als Cloudflare Workers-project met statische assets (wrangler-config erbij)
of via Cloudflare Pages, met het domein dat al op Cloudflare-DNS staat.
