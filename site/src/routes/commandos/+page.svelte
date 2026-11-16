<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { COMMANDOS, CATEGORIEEN, type Categorie } from '#lib/commands.js';
	import { SITE_URL } from '#lib/config.js';
	import { motion, minderBeweging, reveal } from '#lib/motion.js';
	import SiteNav from '#lib/components/SiteNav.svelte';
	import InviteButton from '#lib/components/InviteButton.svelte';
	import SiteFooter from '#lib/components/SiteFooter.svelte';

	let zoek = $state('');
	let filter = $state<Categorie | 'alle'>('alle');
	let lijst: HTMLElement;
	let titel: HTMLHeadingElement;

	const zichtbaar = $derived(
		COMMANDOS.filter((c) => {
			if (filter !== 'alle' && c.categorie !== filter) return false;
			const q = zoek.trim().toLowerCase();
			if (!q) return true;
			return (
				c.naam.toLowerCase().includes(q) ||
				c.uitleg.toLowerCase().includes(q) ||
				(c.opties ?? []).some((o) => o.naam.includes(q))
			);
		})
	);

	const groepen = $derived(
		CATEGORIEEN.map((cat) => ({ ...cat, items: zichtbaar.filter((c) => c.categorie === cat.id) })).filter(
			(g) => g.items.length > 0
		)
	);

	/** Animeer de lijst naar de nieuwe stand (GSAP Flip) als het filter of de zoekterm verandert. */
	async function metFlip(wijzig: () => void) {
		if (!lijst || minderBeweging()) {
			wijzig();
			return;
		}
		const { Flip } = motion();
		const staat = Flip.getState(lijst.querySelectorAll('[data-flip]'));
		wijzig();
		await tick();
		Flip.from(staat, {
			duration: 0.5,
			ease: 'power3.inOut',
			absolute: true,
			nested: true,
			onEnter: (els) => motion().gsap.fromTo(els, { autoAlpha: 0, y: 20 }, { autoAlpha: 1, y: 0, duration: 0.4 })
		});
	}

	function kies(id: Categorie | 'alle') {
		if (id === filter) return;
		metFlip(() => (filter = id));
	}

	let zoekTimer: ReturnType<typeof setTimeout>;
	function typ(e: Event) {
		const waarde = (e.currentTarget as HTMLInputElement).value;
		clearTimeout(zoekTimer);
		zoekTimer = setTimeout(() => metFlip(() => (zoek = waarde)), 120);
	}

	onMount(() => {
		const { gsap, SplitText } = motion();
		const items = document.querySelectorAll('[data-hero]');
		if (minderBeweging()) {
			gsap.set(items, { autoAlpha: 1 });
			return;
		}
		const ctx = gsap.context(() => {
			gsap.set(items, { autoAlpha: 1 });
			const split = SplitText.create(titel, { type: 'chars', mask: 'chars' });
			gsap
				.timeline({ defaults: { ease: 'power4.out' } })
				.from(split.chars, { yPercent: 110, duration: 0.9, stagger: 0.04 })
				.from('[data-hero="sub"] > *', { autoAlpha: 0, y: 20, stagger: 0.08, duration: 0.7 }, 0.3);
		});
		return () => ctx.revert();
	});
</script>

<svelte:head>
	<title>Commando's · Broodkast</title>
	<meta name="description" content="Alle slash-commando's van Broodkast: meldingen instellen en beheren, en een paar voor de lol." />
	<link rel="canonical" href="{SITE_URL}/commandos" />
</svelte:head>

<div class="kopband">
	<div class="gloed" aria-hidden="true"></div>
	<SiteNav />
	<div class="wrap intro">
		<h1 class="display" bind:this={titel} data-hero>Commando's</h1>
		<div class="sub" data-hero="sub">
			<p>
				Alles wat Broodkast kan, als slash-commando in Discord. Typ <strong>/</strong> in een kanaal en kies Broodkast; Discord
				toont dan dezelfde lijst.
			</p>
			<div class="zoekrij">
				<label class="sr-only" for="zoek">Zoek een commando</label>
				<input id="zoek" type="search" placeholder="Zoek een commando" autocomplete="off" oninput={typ} />
				<div class="filters" role="group" aria-label="Filter op categorie">
					<button type="button" class:actief={filter === 'alle'} aria-pressed={filter === 'alle'} onclick={() => kies('alle')}
						>Alle <span>{COMMANDOS.length}</span></button
					>
					{#each CATEGORIEEN as cat (cat.id)}
						<button
							type="button"
							class:actief={filter === cat.id}
							aria-pressed={filter === cat.id}
							onclick={() => kies(cat.id)}
							>{cat.naam} <span>{COMMANDOS.filter((c) => c.categorie === cat.id).length}</span></button
						>
					{/each}
				</div>
			</div>
		</div>
	</div>
</div>

<main class="wrap lijst" bind:this={lijst}>
	{#each groepen as groep (groep.id)}
		<section class="groep" data-flip data-flip-id="groep-{groep.id}">
			<div class="groepkop">
				<h2>{groep.naam}</h2>
				<p>{groep.uitleg}</p>
			</div>
			<div class="kaarten" data-reveal use:reveal={{ kinderen: true, y: 30, stagger: 0.06 }}>
				{#each groep.items as c (c.naam)}
					<article data-flip data-flip-id={c.naam}>
						<div class="naam">{c.naam}</div>
						<p class="uitleg">{c.uitleg}</p>
						{#if c.opties?.length}
							<dl>
								{#each c.opties as o (o.naam)}
									<div>
										<dt>{o.naam}{#if o.verplicht}<span class="verplicht">verplicht</span>{/if}</dt>
										<dd>{o.uitleg}</dd>
									</div>
								{/each}
							</dl>
						{/if}
						<div class="meta">
							<span>{c.waar === 'overal' ? 'Overal, ook in DM' : 'Alleen in servers'}</span>
							{#if c.recht}<span>Recht: {c.recht}</span>{/if}
						</div>
					</article>
				{/each}
			</div>
		</section>
	{:else}
		<div class="leeg">
			<p>Geen commando gevonden voor “{zoek}”.</p>
			<button type="button" onclick={() => metFlip(() => { zoek = ''; filter = 'alle'; (document.getElementById('zoek') as HTMLInputElement).value = ''; })}>
				Toon alles
			</button>
		</div>
	{/each}

	<div class="nog">
		<p>Er komen geregeld commando's bij. Deze lijst groeit mee.</p>
		<InviteButton size="lg" />
	</div>
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
		inset: -40% -10% auto auto;
		width: 70%;
		aspect-ratio: 1;
		border-radius: 50%;
		background: radial-gradient(
			closest-side,
			rgb(255 212 90 / 0.55),
			rgb(255 106 43 / 0.45) 35%,
			rgb(255 46 136 / 0.35) 60%,
			rgb(140 60 242 / 0.15) 80%,
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
		gap: 28px;
		padding-block: 64px 56px;
	}
	h1 {
		font-size: clamp(64px, 11vw, 168px);
		line-height: 0.85;
	}
	.sub {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}
	.sub p {
		max-width: 56ch;
		color: var(--text-soft);
	}
	.zoekrij {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		align-items: center;
	}
	input {
		flex: 1 1 280px;
		min-width: 0;
		font: 500 16px var(--font-body);
		color: var(--text);
		background: rgb(11 8 6 / 0.7);
		border: 1px solid var(--line-strong);
		border-radius: var(--radius-sm);
		padding: 14px 16px;
	}
	input::placeholder {
		color: var(--text-muted);
	}
	.filters {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}
	.filters button {
		font: 600 14px var(--font-body);
		color: var(--text-soft);
		background: rgb(11 8 6 / 0.6);
		border: 1px solid var(--line-strong);
		border-radius: 999px;
		padding: 10px 16px;
		cursor: pointer;
		transition:
			background 0.2s,
			color 0.2s,
			border-color 0.2s;
	}
	.filters button span {
		color: var(--text-muted);
		margin-left: 4px;
		font-variant-numeric: tabular-nums;
	}
	.filters button:hover {
		border-color: var(--text);
		color: var(--text);
	}
	.filters button.actief {
		background: var(--accent);
		border-color: var(--accent);
		color: var(--secondary);
	}
	.filters button.actief span {
		color: var(--secondary);
	}
	.lijst {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 72px;
		padding-block: 72px var(--section);
	}
	.groep {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}
	.groepkop {
		display: flex;
		align-items: baseline;
		gap: 16px;
		flex-wrap: wrap;
		border-bottom: 1px solid var(--line);
		padding-bottom: 14px;
	}
	.groepkop h2 {
		font-family: var(--font-display);
		font-weight: 900;
		font-size: 44px;
		text-transform: uppercase;
		line-height: 1;
	}
	.groepkop p {
		color: var(--text-muted);
	}
	.kaarten {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(min(100%, 340px), 1fr));
		gap: 16px;
	}
	article {
		background: var(--surface);
		border: 1px solid var(--line);
		border-radius: var(--radius-md);
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		min-width: 0;
		transition: border-color 0.25s;
	}
	article:hover {
		border-color: rgb(255 106 43 / 0.6);
	}
	.naam {
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 30px;
		line-height: 1;
		text-transform: uppercase;
		color: var(--main);
		overflow-wrap: anywhere;
	}
	.uitleg {
		color: var(--text-soft);
	}
	dl {
		margin: 0;
		display: grid;
		gap: 8px;
		font-size: 14px;
	}
	dl div {
		display: grid;
		grid-template-columns: minmax(0, 7.5em) minmax(0, 1fr);
		gap: 10px;
	}
	dt {
		font-weight: 600;
	}
	.verplicht {
		display: block;
		font-size: 11px;
		font-weight: 600;
		color: var(--accent);
	}
	dd {
		margin: 0;
		color: var(--text-muted);
	}
	.meta {
		margin-top: auto;
		padding-top: 14px;
		border-top: 1px solid var(--line);
		display: flex;
		flex-wrap: wrap;
		gap: 6px 16px;
		font-size: 13px;
		color: var(--text-muted);
	}
	.leeg {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 16px;
		padding: 32px;
		border: 1px dashed var(--line-strong);
		border-radius: var(--radius-md);
	}
	.leeg button {
		font: 600 14px var(--font-body);
		color: var(--text);
		background: none;
		border: 1px solid var(--line-strong);
		border-radius: var(--radius-sm);
		padding: 10px 14px;
		cursor: pointer;
	}
	.nog {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 20px;
		padding-top: 32px;
		border-top: 1px solid var(--line);
	}
	.nog p {
		color: var(--text-soft);
	}
</style>
