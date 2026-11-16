import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import { SplitText } from 'gsap/SplitText';
import { Flip } from 'gsap/Flip';

let geregistreerd = false;

/** GSAP met plugins; alleen in de browser aanroepen (onMount of een action). */
export function motion() {
	if (!geregistreerd) {
		gsap.registerPlugin(ScrollTrigger, SplitText, Flip);
		geregistreerd = true;
	}
	return { gsap, ScrollTrigger, SplitText, Flip };
}

export function minderBeweging(): boolean {
	return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

interface RevealOpties {
	/** Animeer de kinderen na elkaar in plaats van het element zelf. */
	kinderen?: boolean;
	y?: number;
	stagger?: number;
}

/**
 * Laat een element (of zijn kinderen) inschuiven zodra het in beeld komt.
 * Gebruik met `data-reveal` op het element: dat houdt het verborgen tot GSAP het overneemt.
 */
export function reveal(node: HTMLElement, opties: RevealOpties = {}) {
	const { gsap } = motion();
	const { kinderen = false, y = 40, stagger = 0.08 } = opties;

	if (minderBeweging()) {
		gsap.set(node, { autoAlpha: 1 });
		return {};
	}

	const ctx = gsap.context(() => {
		const doel = kinderen ? Array.from(node.children) : node;
		if (kinderen) gsap.set(node, { autoAlpha: 1 });
		gsap.fromTo(
			doel,
			{ autoAlpha: 0, y },
			{
				autoAlpha: 1,
				y: 0,
				duration: 0.9,
				ease: 'power3.out',
				stagger,
				scrollTrigger: { trigger: node, start: 'top 85%', once: true }
			}
		);
	}, node);

	return {
		destroy() {
			ctx.revert();
		}
	};
}
