<script lang="ts">
	import { SITE_DESCRIPTION, SITE_URL } from '#lib/config.js';
	import { COMMANDOS } from '#lib/commands.js';
	import { reveal } from '#lib/motion.js';
	import SiteNav from '#lib/components/SiteNav.svelte';
	import Hero from '#lib/components/Hero.svelte';
	import ThermalHud from '#lib/components/ThermalHud.svelte';
	import FeatureCard from '#lib/components/FeatureCard.svelte';
	import CommandChip from '#lib/components/CommandChip.svelte';
	import CommandDemo from '#lib/components/CommandDemo.svelte';
	import KoeDemo from '#lib/components/KoeDemo.svelte';
	import MuntDemo from '#lib/components/MuntDemo.svelte';
	import CtaBand from '#lib/components/CtaBand.svelte';
	import SiteFooter from '#lib/components/SiteFooter.svelte';

	const integraties = [
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
	<title>Broodkast · Discord-bot voor feeds, Twitch en webhooks</title>
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
			<h2 class="display">Feeds, Twitch en webhooks</h2>
			<p>
				Kies een kanaal en, als je wilt, een rol om te pingen. <CommandChip naam="/volglijst" /> toont wat je server volgt,
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
		<div class="kop" data-reveal use:reveal={{ kinderen: true }}>
			<h2 class="display">Voor tussendoor</h2>
			<p>Werken ook in je DM's en privégroepen als je Broodkast aan je eigen account toevoegt. Probeer ze hier.</p>
		</div>
		<div class="cmds" data-reveal use:reveal={{ kinderen: true, stagger: 0.1 }}>
			<CommandDemo naam="/koepraat" uitleg="Een koe zegt wat jij typt, in een tekstballon.">
				<KoeDemo />
			</CommandDemo>
			<CommandDemo naam="/muntgooier" uitleg="Kop of munt, voor elke knoop die doorgehakt moet worden.">
				<MuntDemo />
			</CommandDemo>
			<a class="alles" href="/commandos">
				<span class="getal">{aantal}</span>
				<span class="label">Alle commando's</span>
				<span class="pijl" aria-hidden="true">→</span>
			</a>
		</div>
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
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1px;
		background: var(--line);
		border: 1px solid var(--line);
		border-radius: var(--radius-md);
		overflow: hidden;
	}
	.cmds {
		display: grid;
		grid-template-columns: 1.3fr 1fr 1fr;
		gap: 16px;
	}
	.alles {
		position: relative;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 24px;
		padding: 28px;
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
		width: 90%;
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
		font-size: 96px;
		line-height: 0.85;
		color: var(--main);
		font-variant-numeric: tabular-nums;
	}
	.label {
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 32px;
		text-transform: uppercase;
		line-height: 1;
	}
	.pijl {
		position: relative;
		z-index: 1;
		font-size: 28px;
		transition: transform 0.3s;
	}
	.alles:hover .pijl {
		transform: translateX(8px);
	}
	@media (max-width: 900px) {
		.raster3,
		.cmds {
			grid-template-columns: minmax(0, 1fr);
		}
	}
</style>
