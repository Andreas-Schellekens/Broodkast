<script lang="ts">
	import { SITE_DESCRIPTION, SITE_URL } from '#lib/config.js';
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
			label: 'Controle elke',
			waarde: '10 min',
			titel: 'Feeds',
			tekst:
				'Elke RSS- of Atom-feed: YouTube-kanalen, subreddits, GitHub-releases, blogs. Oude berichten worden nooit opnieuw gepost.',
			commando: '/volg rss',
			limiet: 'max. 5 per ronde'
		},
		{
			label: 'Controle elke',
			waarde: '2 min',
			titel: 'Twitch',
			tekst: 'Een melding zodra een streamer live gaat, met titel, spel en eventueel een rol-ping. Eén melding per stream.',
			commando: '/volg twitch',
			limiet: 'per streamer'
		},
		{
			label: 'Vertraging',
			waarde: '0 sec',
			titel: 'Webhooks',
			tekst: 'Een geheime URL voor je eigen diensten. Titel, tekst, link, afbeelding en kleur komen meteen in je kanaal.',
			commando: '/volg webhook',
			limiet: '1 bericht per 2 s'
		}
	];
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
		<div class="kop">
			<h2 class="display">Drie bronnen. Altijd warm.</h2>
			<p>
				Kies een kanaal en, als je wilt, een rol om te pingen. <CommandChip naam="/volglijst" /> toont wat je server volgt, tot
				50 bronnen. <CommandChip naam="/ontvolg" /> haalt er een weg.
			</p>
		</div>
		<div class="raster3">
			{#each integraties as item (item.titel)}
				<FeatureCard {...item} />
			{/each}
		</div>
	</div>
</section>

<section id="commandos" class="sectie">
	<div class="wrap stapel">
		<div class="kop">
			<h2 class="display">Voor tussendoor</h2>
			<p>Werken ook in je DM's en privégroepen als je Broodkast aan je eigen account toevoegt. Probeer ze hier.</p>
		</div>
		<div class="cmds">
			<CommandDemo naam="/koepraat" uitleg="Een koe zegt wat jij typt, in een tekstballon.">
				<KoeDemo />
			</CommandDemo>
			<CommandDemo naam="/muntgooier" uitleg="Kop of munt, voor elke knoop die doorgehakt moet worden.">
				<MuntDemo />
			</CommandDemo>
			<CommandDemo naam="/apod" uitleg="De astronomiefoto van de dag van NASA, met uitleg erbij." />
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
	@media (max-width: 900px) {
		.raster3,
		.cmds {
			grid-template-columns: minmax(0, 1fr);
		}
	}
</style>
