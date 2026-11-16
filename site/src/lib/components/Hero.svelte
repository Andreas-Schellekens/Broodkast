<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import InviteButton from './InviteButton.svelte';
	import { motion, minderBeweging } from '#lib/motion.js';

	/** `kop`: de navigatie, die over de hittegloed ligt. `beeld`: de rechterkolom. */
	let { kop, beeld }: { kop?: Snippet; beeld?: Snippet } = $props();

	let sectie: HTMLElement;
	let gloed: HTMLDivElement;
	let titel: HTMLHeadingElement;

	onMount(() => {
		const { gsap, SplitText } = motion();
		const hero = sectie.querySelectorAll('[data-hero]');

		if (minderBeweging()) {
			gsap.set(hero, { autoAlpha: 1 });
			return;
		}

		const ctx = gsap.context(() => {
			gsap.set(hero, { autoAlpha: 1 });
			const split = SplitText.create(titel, { type: 'lines,words', mask: 'lines' });
			// Na het splitsen staat elk woord in een eigen laag; geef de woorden van het verloop elk het verloop.
			titel.querySelectorAll('.hot div').forEach((w) => w.classList.add('hot'));

			const tl = gsap.timeline({ defaults: { ease: 'power4.out' } });
			tl.from(gloed, { autoAlpha: 0, scale: 1.25, xPercent: 8, duration: 2.2, ease: 'power2.out' }, 0)
				.from(split.words, { yPercent: 110, duration: 1.1, stagger: 0.06 }, 0.15)
				.from('[data-hero="lead"]', { autoAlpha: 0, y: 24, duration: 0.9 }, 0.7)
				.from('[data-hero="acties"] > *', { autoAlpha: 0, y: 24, duration: 0.8, stagger: 0.1 }, 0.85)
				.from('[data-hero="beeld"]', { autoAlpha: 0, scale: 0.92, duration: 1.4 }, 0.4);

			// De gloed ademt zacht na.
			gsap.to(gloed, {
				scale: 1.06,
				xPercent: -2,
				duration: 7,
				ease: 'sine.inOut',
				yoyo: true,
				repeat: -1,
				delay: 2.2
			});

			// Bij scrollen zakt de gloed weg en vervaagt hij.
			gsap.to(gloed, {
				yPercent: 18,
				opacity: 0.35,
				ease: 'none',
				scrollTrigger: { trigger: sectie, start: 'top top', end: 'bottom top', scrub: true }
			});

			// Muisparallax op apparaten met een muis.
			if (window.matchMedia('(pointer: fine)').matches) {
				const gx = gsap.quickTo(gloed, 'x', { duration: 1.2, ease: 'power3' });
				const gy = gsap.quickTo(gloed, 'y', { duration: 1.2, ease: 'power3' });
				const bx = gsap.quickTo('[data-hero="beeld"]', 'x', { duration: 1, ease: 'power3' });
				const by = gsap.quickTo('[data-hero="beeld"]', 'y', { duration: 1, ease: 'power3' });
				const beweeg = (e: PointerEvent) => {
					const rx = e.clientX / window.innerWidth - 0.5;
					const ry = e.clientY / window.innerHeight - 0.5;
					gx(rx * -40);
					gy(ry * -30);
					bx(rx * 18);
					by(ry * 14);
				};
				sectie.addEventListener('pointermove', beweeg);
				return () => sectie.removeEventListener('pointermove', beweeg);
			}
		}, sectie);

		return () => ctx.revert();
	});
</script>

<section class="hero" id="top" bind:this={sectie}>
	<!-- GSAP beweegt de wrapper; de img zelf blijft gespiegeld. -->
	<div class="gloed" bind:this={gloed} aria-hidden="true">
		<img src="/thermal.webp" alt="" width="1800" height="1200" fetchpriority="high" />
	</div>
	{#if kop}{@render kop()}{/if}
	<div class="wrap inhoud">
		<div class="tekst">
			<h1 class="display" bind:this={titel} data-hero="titel">Vers uit de oven. <span class="hot">Recht in je kanaal.</span></h1>
			<p class="lead" data-hero="lead">
				Broodkast zet nieuwe video's, posts, releases, Twitch-streams en berichten van je eigen diensten in het Discord-kanaal
				dat jij kiest. Per bron is dat één commando.
			</p>
			<div class="acties" data-hero="acties">
				<InviteButton size="lg" />
				<InviteButton variant="ghost" href="#integraties" label="Bekijk wat hij doet" />
			</div>
		</div>
		{#if beeld}
			<div class="beeld" data-hero="beeld">{@render beeld()}</div>
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
		right: -4%;
		bottom: -4%;
		width: 68%;
		height: 108%;
		pointer-events: none;
		will-change: transform;
	}
	.gloed img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: left bottom;
		transform: scaleX(-1);
	}
	.hero::before {
		content: '';
		position: absolute;
		inset: 0;
		z-index: 1;
		background: linear-gradient(90deg, var(--bg) 0%, var(--bg) 36%, rgb(11 8 6 / 0) 58%);
		pointer-events: none;
	}
	.inhoud {
		position: relative;
		z-index: 2;
		display: grid;
		grid-template-columns: minmax(0, 1.15fr) minmax(0, 1fr);
		gap: 48px;
		align-items: center;
		padding-block: 96px 140px;
	}
	.tekst {
		display: flex;
		flex-direction: column;
		gap: 26px;
		min-width: 0;
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
