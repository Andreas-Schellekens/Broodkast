<script lang="ts">
	import { onMount } from 'svelte';
	import Logo from './Logo.svelte';
	import { motion, minderBeweging } from '#lib/motion.js';

	const schaal = ['230°', '160°', '90°', '20°'];

	let hud: HTMLDivElement;
	let temp: HTMLSpanElement;
	const eind = 212.4;

	onMount(() => {
		if (minderBeweging()) return;
		const { gsap } = motion();

		const ctx = gsap.context(() => {
			// Temperatuur loopt op van kamertemperatuur naar korst.
			const meting = { t: 20 };
			gsap.to(meting, {
				t: eind,
				duration: 2.4,
				delay: 0.6,
				ease: 'power2.out',
				onUpdate: () => {
					temp.textContent = meting.t.toFixed(1).replace('.', ',');
				}
			});

			gsap.from('.balk', { scaleY: 0, transformOrigin: 'bottom', duration: 1.6, delay: 0.4, ease: 'power3.out' });
			gsap.from('.getallen span', { autoAlpha: 0, x: 8, stagger: 0.12, delay: 0.8 });

			// Het richtkruis zoekt de kast en vergrendelt.
			gsap.from('.kruis', { x: 120, y: -90, autoAlpha: 0, duration: 1.6, delay: 0.5, ease: 'expo.out' });
			gsap.to('.ring', { scale: 1.6, autoAlpha: 0, duration: 1.4, repeat: -1, ease: 'power1.out', delay: 2 });

			// De kast zweeft en de antenne zendt.
			gsap.to('.kast', { y: -14, duration: 3.2, ease: 'sine.inOut', yoyo: true, repeat: -1 });
			gsap.fromTo(
				'.golf',
				{ scale: 0.4, autoAlpha: 0.9 },
				{ scale: 1.8, autoAlpha: 0, duration: 2.2, ease: 'power1.out', stagger: 0.7, repeat: -1 }
			);
		}, hud);

		return () => ctx.revert();
	});
</script>

<!-- Warmtecamera-overlay: temperatuurschaal, richtkruis en de gloeiende broodkast. Puur decoratief. -->
<div class="hud" bind:this={hud} aria-hidden="true">
	<div class="schaal">
		<div class="getallen">
			{#each schaal as t (t)}<span>{t}</span>{/each}
		</div>
		<div class="balk"></div>
	</div>

	<div class="kast">
		<div class="golven">
			<span class="golf"></span><span class="golf"></span><span class="golf"></span>
		</div>
		<Logo size={300} />
	</div>

	<div class="kruis">
		<span class="h"></span><span class="v"></span><span class="ring"></span><span class="ring vast"></span>
		<span class="label"><span bind:this={temp}>{String(eind).replace('.', ',')}</span> °C <small>korst</small></span>
	</div>
</div>

<style>
	.hud {
		position: relative;
		height: 460px;
	}
	.schaal {
		position: absolute;
		right: 0;
		top: 10px;
		height: 300px;
		display: flex;
		gap: 10px;
	}
	.getallen {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		text-align: right;
		font-family: var(--font-display);
		font-weight: 600;
		font-size: 15px;
		font-variant-numeric: tabular-nums;
	}
	.balk {
		width: 12px;
		border-radius: 3px;
		background: linear-gradient(180deg, #fff3c4, #ffd45a 12%, #ffa23a 30%, #ff5a3c 48%, #f02d8c 66%, #8c3cf2 84%, #4a63ff);
	}
	.kast {
		position: absolute;
		left: 8%;
		top: 90px;
		filter: drop-shadow(0 30px 60px rgb(255 46 136 / 0.35));
	}
	.golven {
		position: absolute;
		/* boven het antennebolletje: (89, 5) in een viewBox van 104 × 82 */
		left: calc(300px * 89 / 104);
		top: calc(300px * 82 / 104 * 5 / 82);
	}
	.golf {
		position: absolute;
		left: -40px;
		top: -40px;
		width: 80px;
		height: 80px;
		border: 2px solid var(--accent);
		border-radius: 50%;
		opacity: 0;
	}
	/* Richtkruis op het heetste punt van de kast (35 %, 75 % van het logo). */
	.kruis {
		position: absolute;
		left: calc(8% + 105px);
		top: 267px;
	}
	.h,
	.v,
	.ring {
		position: absolute;
		background: var(--text);
	}
	.h {
		left: -22px;
		width: 44px;
		height: 1px;
	}
	.v {
		top: -22px;
		width: 1px;
		height: 44px;
	}
	.ring {
		left: -9px;
		top: -9px;
		width: 18px;
		height: 18px;
		border: 1px solid var(--text);
		border-radius: 50%;
		background: none;
	}
	.label {
		position: absolute;
		left: -40px;
		top: 64px;
		white-space: nowrap;
		background: rgb(11 8 6 / 0.72);
		border: 1px solid var(--line-strong);
		border-radius: 4px;
		padding: 6px 10px;
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 22px;
		line-height: 1;
		font-variant-numeric: tabular-nums;
	}
	.label small {
		font-family: var(--font-body);
		font-weight: 500;
		font-size: 12px;
		color: var(--text-soft);
		margin-left: 4px;
	}
	@media (max-width: 900px) {
		.hud {
			display: none;
		}
	}
</style>
