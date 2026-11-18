<script lang="ts">
	import { SITE_DESCRIPTION, SITE_URL } from '#lib/config.js';
	import { COMMANDOS } from '#lib/commands.js';
	import { reveal } from '#lib/motion.js';
	import SiteNav from '#lib/components/SiteNav.svelte';
	import Hero from '#lib/components/Hero.svelte';
	import ThermalHud from '#lib/components/ThermalHud.svelte';
	import FeatureCard from '#lib/components/FeatureCard.svelte';
	import CommandChip from '#lib/components/CommandChip.svelte';
	import CtaBand from '#lib/components/CtaBand.svelte';
	import SiteFooter from '#lib/components/SiteFooter.svelte';

	const integraties = [
		{
			titel: 'VRT NWS',
			tekst: 'Het nieuws in je server, zonder een feed-URL te zoeken. Kies alle artikels of alleen de hoofdpunten.',
			voorbeelden: ['Alle artikels', 'Hoofdpunten'],
			commando: '/volg nieuws'
		},
		{
			titel: 'Gratis games',
			tekst: 'Een melding zodra een game tijdelijk gratis is. Alleen nieuwe giveaways, met de einddatum als die bekend is.',
			voorbeelden: ['Epic Games Store', 'Steam'],
			commando: '/volg gratisgames'
		},
		{
			titel: 'Feeds',
			tekst: 'Elke RSS- of Atom-feed. Alleen nieuwe berichten komen in je kanaal; oude worden nooit opnieuw gepost.',
			voorbeelden: ['YouTube-kanalen', 'Subreddits', 'GitHub-releases', 'Blogs'],
			commando: '/volg rss'
		},
		{
			titel: 'Twitch',
			tekst: 'Een melding zodra een streamer live gaat, met titel en spel. Eén melding per stream.',
			voorbeelden: ['Live-melding', 'Rol-ping'],
			commando: '/volg twitch'
		},
		{
			titel: 'Webhooks',
			tekst: 'Een geheime URL voor je eigen diensten. Wat een server, script of formulier daarheen stuurt, staat in je kanaal.',
			voorbeelden: ['Titel', 'Tekst', 'Link', 'Afbeelding', 'Kleur'],
			commando: '/volg webhook'
		}
	];

	const aantal = COMMANDOS.length;
</script>

<svelte:head>
	<title>Broodkast · Nederlandstalige Discord-bot voor nieuws, games en meldingen</title>
	<meta name="description" content={SITE_DESCRIPTION} />
	<link rel="canonical" href={SITE_URL} />
	<meta property="og:title" content="Broodkast" />
	<meta property="og:description" content={SITE_DESCRIPTION} />
	<meta property="og:url" content={SITE_URL} />
	<meta property="og:type" content="website" />
</svelte:head>

<Hero>
	{#snippet kop()}<SiteNav />{/snippet}
	{#snippet beeld()}<ThermalHud />{/snippet}
</Hero>

<section id="integraties" class="sectie">
	<div class="wrap stapel">
		<div class="kop" data-reveal use:reveal={{ kinderen: true }}>
			<h2 class="display">Wat Broodkast volgt</h2>
			<p>
				Volledig in het Nederlands. Kies een kanaal en, als je wilt, een rol om te pingen. <CommandChip naam="/volglijst" /> toont wat je server volgt,
				tot 50 bronnen. <CommandChip naam="/ontvolg" /> haalt er een weg.
			</p>
		</div>
		<div class="raster3" data-reveal use:reveal={{ kinderen: true, y: 60, stagger: 0.12 }}>
			{#each integraties as item (item.titel)}
				<FeatureCard {...item} />
			{/each}
		</div>
	</div>
</section>

<section id="commandos" class="sectie">
	<div class="wrap stapel">
		<a class="alles" href="/commandos" data-reveal use:reveal>
			<span class="getal">{aantal}</span>
			<span class="tekst">
				<span class="label">Commando's</span>
				<span class="uitleg">Meldingen, herinneringen, het weer, dobbelstenen en meer. Bekijk ze allemaal.</span>
			</span>
			<span class="pijl" aria-hidden="true">→</span>
		</a>
	</div>
</section>

<CtaBand />
<SiteFooter />

<style>
	.sectie {
		border-top: 1px solid var(--line);
	}
	.stapel {
		display: flex;
		flex-direction: column;
		gap: 52px;
		padding-block: var(--section);
	}
	.kop {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: flex-end;
		gap: 24px;
	}
	.kop h2 {
		font-size: clamp(44px, 5.6vw, 80px);
		max-width: 12ch;
	}
	.kop p {
		max-width: 44ch;
		color: var(--text-muted);
	}
	.raster3 {
		display: grid;
		grid-template-columns: repeat(6, minmax(0, 1fr));
		gap: 1px;
		background: var(--line);
		border: 1px solid var(--line);
		border-radius: var(--radius-md);
		overflow: hidden;
	}
	.alles {
		position: relative;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 20px 40px;
		padding: 40px 44px;
		border-radius: var(--radius-md);
		background: var(--surface);
		border: 1px solid var(--line);
		text-decoration: none;
		overflow: hidden;
		transition: border-color 0.3s;
	}
	.alles::after {
		content: '';
		position: absolute;
		inset: auto -30% -60% auto;
		width: 55%;
		aspect-ratio: 1;
		border-radius: 50%;
		background: radial-gradient(closest-side, rgb(255 46 136 / 0.55), rgb(255 106 43 / 0.25), transparent);
		transition: transform 0.6s ease;
		pointer-events: none;
	}
	.alles:hover {
		border-color: var(--accent);
	}
	.alles:hover::after {
		transform: scale(1.4);
	}
	.getal {
		font-family: var(--font-display);
		font-weight: 900;
		font-size: clamp(88px, 11vw, 160px);
		line-height: 0.85;
		color: var(--main);
		font-variant-numeric: tabular-nums;
	}
	.tekst {
		position: relative;
		z-index: 1;
		flex: 1 1 280px;
		display: flex;
		flex-direction: column;
		gap: 10px;
		min-width: 0;
	}
	.label {
		font-family: var(--font-display);
		font-weight: 800;
		font-size: clamp(36px, 4.4vw, 60px);
		text-transform: uppercase;
		line-height: 1;
	}
	.uitleg {
		max-width: 44ch;
		color: var(--text-muted);
	}
	.pijl {
		position: relative;
		z-index: 1;
		font-size: 44px;
		transition: transform 0.3s;
	}
	.alles:hover .pijl {
		transform: translateX(8px);
	}
	/* Bovenste rij: twee brede kaarten (nieuws, games); daaronder drie. */
	.raster3 > :global(*) {
		grid-column: span 2;
	}
	.raster3 > :global(:nth-child(-n + 2)) {
		grid-column: span 3;
	}
	@media (max-width: 900px) {
		.raster3 > :global(*),
		.raster3 > :global(:nth-child(-n + 2)) {
			grid-column: auto;
		}
		.raster3 {
			grid-template-columns: minmax(0, 1fr);
		}
	}
</style>
