<script lang="ts">
	import type { Snippet } from 'svelte';
	import InviteButton from './InviteButton.svelte';

	/** `kop`: de navigatie, die over de hittegloed ligt. `beeld`: de rechterkolom. */
	let { kop, beeld }: { kop?: Snippet; beeld?: Snippet } = $props();
</script>

<section class="hero" id="top">
	<img class="gloed" src="/thermal.webp" alt="" aria-hidden="true" width="1800" height="1200" fetchpriority="high" />
	{#if kop}{@render kop()}{/if}
	<div class="wrap inhoud">
		<div class="tekst">
			<div class="eyebrow tag"><span class="stip" aria-hidden="true"></span>Discord-bot · in het Nederlands</div>
			<h1 class="display">Vers uit de oven. <span class="hot">Recht in je kanaal.</span></h1>
			<p class="lead">
				Broodkast zet nieuwe video's, posts, releases, Twitch-streams en berichten van je eigen diensten in het Discord-kanaal
				dat jij kiest, terwijl ze nog warm zijn.
			</p>
			<div class="acties">
				<InviteButton size="lg" />
				<InviteButton variant="ghost" href="#integraties" label="Bekijk wat hij doet" />
			</div>
		</div>
		{#if beeld}
			<div class="beeld">{@render beeld()}</div>
		{/if}
	</div>
</section>

<style>
	.hero {
		position: relative;
		overflow: hidden;
	}
	/* De gloed blijft in de rechterhelft; links houdt een zwarte overgang de tekst en de knop leesbaar. */
	.gloed {
		position: absolute;
		right: 0;
		bottom: 0;
		width: 64%;
		height: 100%;
		object-fit: cover;
		object-position: left bottom;
		transform: scaleX(-1);
		pointer-events: none;
	}
	.hero::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, var(--bg) 0%, var(--bg) 36%, rgb(11 8 6 / 0) 58%);
		pointer-events: none;
		z-index: 1;
	}
	.inhoud {
		position: relative;
		z-index: 2;
		display: grid;
		grid-template-columns: minmax(0, 1.15fr) minmax(0, 1fr);
		gap: 48px;
		align-items: start;
		padding-block: 96px 140px;
	}
	.tekst {
		display: flex;
		flex-direction: column;
		gap: 26px;
		min-width: 0;
	}
	.tag {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 13px;
	}
	.stip {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--accent);
	}
	h1 {
		font-size: clamp(56px, 8.4vw, 124px);
		line-height: 0.88;
	}
	.lead {
		max-width: 42ch;
		font-size: 19px;
		color: var(--text-soft);
	}
	.acties {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 14px;
	}
	.beeld {
		min-width: 0;
	}
	@media (max-width: 900px) {
		.inhoud {
			grid-template-columns: minmax(0, 1fr);
			padding-block: 56px 72px;
		}
		.gloed {
			width: 120%;
			height: 55%;
			right: -20%;
		}
		.hero::before {
			background: linear-gradient(0deg, rgb(11 8 6 / 0) 0%, var(--bg) 55%);
		}
	}
</style>
