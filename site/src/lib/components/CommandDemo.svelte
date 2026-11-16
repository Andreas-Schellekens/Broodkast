<script lang="ts">
	import type { Snippet } from 'svelte';

	let { naam, uitleg, children }: { naam: string; uitleg: string; children?: Snippet } = $props();
</script>

<div class="demo">
	<h3>{naam}</h3>
	<p>{uitleg}</p>
	{#if children}<div class="speel">{@render children()}</div>{/if}
</div>

<style>
	.demo {
		border: 1px solid var(--line);
		border-radius: var(--radius-md);
		padding: 28px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		min-width: 0;
	}
	h3 {
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 32px;
		text-transform: uppercase;
		color: var(--main);
	}
	p {
		color: var(--text-soft);
	}
	.speel {
		margin-top: 6px;
	}
</style>
