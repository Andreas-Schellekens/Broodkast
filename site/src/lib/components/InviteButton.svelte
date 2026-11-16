<script lang="ts">
	import { INVITE_URL } from '#lib/config.js';
	import { motion, minderBeweging } from '#lib/motion.js';

	let {
		variant = 'cta',
		size = 'md',
		href = INVITE_URL,
		label = 'Voeg toe aan Discord'
	}: {
		variant?: 'cta' | 'ghost';
		size?: 'sm' | 'md' | 'lg';
		href?: string;
		label?: string;
	} = $props();

	let knop: HTMLAnchorElement;
	let bezig = false;

	const KLEUREN = ['#FFF3C4', '#FFD45A', '#FF6A2B', '#FF2E88', '#7A2CF0'];

	/**
	 * Klik op de CTA: de knop veert in, een hittegolf en vonken spatten uit het klikpunt,
	 * daarna gaat de browser naar Discord. Ctrl-, Cmd- of middelklik werken gewoon als link.
	 */
	function klik(e: MouseEvent) {
		if (variant !== 'cta' || e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0) return;
		if (minderBeweging()) return;
		e.preventDefault();
		if (bezig) return;
		bezig = true;

		const { gsap } = motion();
		const r = knop.getBoundingClientRect();
		// Bij toetsenbordactivatie is er geen klikpunt; neem dan het midden.
		const x = e.clientX || r.left + r.width / 2;
		const y = e.clientY || r.top + r.height / 2;

		const laag = document.createElement('div');
		laag.className = 'vonklaag';
		laag.setAttribute('aria-hidden', 'true');
		document.body.append(laag);

		// Hittegolf
		const golf = document.createElement('span');
		golf.className = 'vonkgolf';
		golf.style.left = `${x}px`;
		golf.style.top = `${y}px`;
		laag.append(golf);

		// Vonken
		const vonken = Array.from({ length: 22 }, (_, i) => {
			const v = document.createElement('span');
			v.className = 'vonk';
			const d = 4 + Math.random() * 6;
			v.style.cssText = `left:${x}px;top:${y}px;width:${d}px;height:${d}px;background:${KLEUREN[i % KLEUREN.length]};color:${KLEUREN[i % KLEUREN.length]}`;
			laag.append(v);
			return v;
		});

		let weg = false;
		const ga = () => {
			if (weg) return;
			weg = true;
			clearTimeout(vangnet);
			window.location.href = href;
			// Komt de bezoeker terug (bv. via de terugknop), dan staat alles weer klaar.
			setTimeout(() => {
				tl.kill();
				laag.remove();
				gsap.set([knop, knop.querySelector('svg')], { clearProps: 'transform' });
				bezig = false;
			}, 600);
		};
		// Vangnet: in een verborgen tabblad pauzeert de animatie; ga dan toch naar Discord.
		const vangnet = setTimeout(ga, 900);

		const tl = gsap.timeline({ onComplete: ga });

		tl.to(knop, { scale: 0.94, duration: 0.09, ease: 'power2.out' })
			.to(knop, { scale: 1, duration: 0.5, ease: 'elastic.out(1.1, 0.4)' })
			.to(knop.querySelector('svg'), { rotate: 180, duration: 0.5, ease: 'back.out(2)' }, 0)
			.fromTo(
				golf,
				{ scale: 0, autoAlpha: 0.9 },
				{ scale: 1, autoAlpha: 0, duration: 0.6, ease: 'power2.out' },
				0.04
			)
			.to(
				vonken,
				{
					x: () => gsap.utils.random(-170, 170),
					y: () => gsap.utils.random(-150, 70),
					scale: 0,
					duration: () => gsap.utils.random(0.45, 0.75),
					ease: 'power3.out'
				},
				0.04
			);
	}
</script>

<a class="knop {variant} {size}" {href} bind:this={knop} onclick={klik}>
	{#if variant === 'cta'}
		<svg width="18" height="18" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14" /></svg>
	{/if}
	{label}
</a>

<style>
	.knop {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		text-decoration: none;
		font-family: var(--font-body);
		font-weight: 600;
		border-radius: var(--radius-sm);
		white-space: nowrap;
		transition:
			background 0.15s,
			box-shadow 0.2s,
			translate 0.15s,
			border-color 0.15s;
	}
	.md {
		font-size: 15px;
		padding: 15px 22px;
	}
	.sm {
		font-size: 14px;
		padding: 11px 16px;
	}
	.lg {
		font-size: 17px;
		padding: 19px 28px;
	}
	.cta {
		background: var(--cta-bg);
		color: var(--cta-text);
		box-shadow: var(--cta-glow);
	}
	.cta:hover {
		background: var(--cta-bg-hover);
		box-shadow: var(--cta-glow-hover);
		/* translate in plaats van transform: GSAP gebruikt transform voor het klikeffect. */
		translate: 0 -1px;
	}
	.ghost {
		border: 1px solid var(--line-strong);
		color: var(--text);
	}
	.ghost:hover {
		border-color: var(--text);
	}
	/* De vonken hangen aan body, dus globaal stylen. */
	:global(.vonklaag) {
		position: fixed;
		inset: 0;
		z-index: 1000;
		pointer-events: none;
		overflow: hidden;
	}
	:global(.vonk) {
		position: absolute;
		margin: -4px 0 0 -4px;
		border-radius: 50%;
		box-shadow: 0 0 10px currentColor;
	}
	:global(.vonkgolf) {
		position: absolute;
		width: 260px;
		height: 260px;
		margin: -130px 0 0 -130px;
		border-radius: 50%;
		background: radial-gradient(closest-side, rgb(255 212 90 / 0) 55%, rgb(255 106 43 / 0.55) 75%, rgb(255 46 136 / 0) 100%);
	}
	@media (prefers-reduced-motion: no-preference) {
		.cta.lg {
			animation: gloei 3.2s ease-in-out infinite;
		}
		@keyframes gloei {
			50% {
				box-shadow: var(--cta-glow-hover);
			}
		}
	}
	@media (max-width: 520px) {
		.sm {
			font-size: 13px;
			padding: 10px 12px;
		}
	}
</style>
