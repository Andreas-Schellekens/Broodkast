<script lang="ts">
	import type { Snippet } from 'svelte';
	import SiteNav from './SiteNav.svelte';
	import SiteFooter from './SiteFooter.svelte';
	import { SITE_URL } from '#lib/config.js';

	/** Sjabloon voor privacybeleid en voorwaarden: titelband, leesbare tekstkolom, footer. */
	let {
		titel,
		pad,
		omschrijving,
		bijgewerkt,
		children
	}: { titel: string; pad: string; omschrijving: string; bijgewerkt: string; children: Snippet } = $props();
</script>

<svelte:head>
	<title>{titel} · Broodkast</title>
	<meta name="description" content={omschrijving} />
	<link rel="canonical" href="{SITE_URL}{pad}" />
</svelte:head>

<div class="kopband">
	<div class="gloed" aria-hidden="true"></div>
	<SiteNav />
	<div class="wrap intro">
		<h1 class="display">{titel}</h1>
		<p>Laatst bijgewerkt op {bijgewerkt}</p>
	</div>
</div>

<main class="wrap">
	<article class="tekst">
		{@render children()}
	</article>
</main>

<SiteFooter />

<style>
	.kopband {
		position: relative;
		overflow: hidden;
		border-bottom: 1px solid var(--line);
	}
	.gloed {
		position: absolute;
		inset: -60% -15% auto auto;
		width: 55%;
		aspect-ratio: 1;
		border-radius: 50%;
		background: radial-gradient(
			closest-side,
			rgb(255 106 43 / 0.35),
			rgb(255 46 136 / 0.22) 50%,
			rgb(140 60 242 / 0.08) 80%,
			transparent
		);
		filter: blur(20px);
		pointer-events: none;
	}
	.intro {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		gap: 16px;
		padding-block: 56px 48px;
	}
	h1 {
		font-size: clamp(48px, 8vw, 104px);
		line-height: 0.9;
	}
	.intro p {
		color: var(--text-muted);
	}
	main {
		padding-block: 56px var(--section);
	}
	.tekst {
		max-width: 68ch;
		display: flex;
		flex-direction: column;
		gap: 16px;
		color: var(--text-soft);
		line-height: 1.65;
	}
	.tekst :global(h2) {
		margin-top: 28px;
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 30px;
		line-height: 1.05;
		text-transform: uppercase;
		color: var(--text);
	}
	.tekst :global(ul) {
		margin: 0;
		padding-left: 1.2em;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	.tekst :global(strong) {
		color: var(--text);
		font-weight: 600;
	}
	.tekst :global(a) {
		color: var(--main);
		text-underline-offset: 3px;
	}
	.tekst :global(a:hover) {
		color: var(--text);
	}
</style>
