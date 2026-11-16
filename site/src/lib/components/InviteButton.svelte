<script lang="ts">
	import { INVITE_URL } from '#lib/config.js';

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
</script>

<a class="knop {variant} {size}" {href}>
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
			transform 0.15s,
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
		transform: translateY(-1px);
	}
	.ghost {
		border: 1px solid var(--line-strong);
		color: var(--text);
	}
	.ghost:hover {
		border-color: var(--text);
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
