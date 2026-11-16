<script lang="ts">
	import { onMount } from 'svelte';
	import InviteButton from './InviteButton.svelte';
	import { motion, minderBeweging, reveal } from '#lib/motion.js';

	let band: HTMLElement;
	let gloed: HTMLDivElement;

	onMount(() => {
		if (minderBeweging()) return;
		const { gsap } = motion();
		const ctx = gsap.context(() => {
			// De gloed schuift mee met het scrollen en kruipt naar de knop toe.
			gsap.fromTo(
				gloed,
				{ xPercent: -18, scale: 1.15 },
				{ xPercent: 6, scale: 1, ease: 'none', scrollTrigger: { trigger: band, start: 'top bottom', end: 'bottom top', scrub: true } }
			);
		}, band);
		return () => ctx.revert();
	});
</script>

<section bind:this={band}>
	<div class="gloed" bind:this={gloed} aria-hidden="true">
		<img src="/thermal.webp" alt="" width="1800" height="1200" loading="lazy" />
	</div>
	<div class="wrap inhoud" data-reveal use:reveal={{ kinderen: true }}>
		<h2 class="display">Zet de broodkast in je server</h2>
		<p>Je voegt de bot toe via Discord. Daarna volg je je eerste bron met /volg.</p>
		<InviteButton size="lg" />
	</div>
</section>

<style>
	section {
		position: relative;
		overflow: hidden;
		border-top: 1px solid var(--line);
	}
	.gloed {
		position: absolute;
		left: 0;
		top: 0;
		width: 70%;
		height: 100%;
		pointer-events: none;
		will-change: transform;
	}
	.gloed img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: left bottom;
	}
	/* Rechts zwart, zodat de roze knop op donker staat en knalt. */
	section::before {
		content: '';
		position: absolute;
		inset: 0;
		z-index: 1;
		background: linear-gradient(270deg, var(--bg) 0%, var(--bg) 34%, rgb(11 8 6 / 0) 62%);
		pointer-events: none;
	}
	.inhoud {
		position: relative;
		z-index: 2;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		text-align: right;
		gap: 22px;
		padding-block: 140px;
	}
	h2 {
		font-size: clamp(48px, 6vw, 92px);
		line-height: 0.88;
		max-width: 11ch;
	}
	p {
		max-width: 34ch;
		font-weight: 500;
		color: var(--text-soft);
	}
	@media (max-width: 900px) {
		.inhoud {
			align-items: flex-start;
			text-align: left;
			padding-block: 96px;
		}
		.gloed {
			width: 100%;
			opacity: 0.55;
		}
		section::before {
			background: rgb(11 8 6 / 0.35);
		}
	}
</style>
