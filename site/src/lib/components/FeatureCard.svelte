<script lang="ts">
	import CommandChip from './CommandChip.svelte';

	let {
		titel,
		tekst,
		voorbeelden = [],
		commando
	}: {
		titel: string;
		tekst: string;
		voorbeelden?: string[];
		commando: string;
	} = $props();
</script>

<article>
	<h3 class="hot">{titel}</h3>
	<p>{tekst}</p>
	{#if voorbeelden.length}
		<ul>
			{#each voorbeelden as v (v)}<li>{v}</li>{/each}
		</ul>
	{/if}
	<div class="voet"><CommandChip naam={commando} /></div>
</article>

<style>
	article {
		background: var(--surface);
		padding: 32px;
		display: flex;
		flex-direction: column;
		gap: 16px;
		min-width: 0;
		transition: background 0.3s;
	}
	article:hover {
		background: #1f1814;
	}
	h3 {
		font-family: var(--font-display);
		font-weight: 900;
		font-size: clamp(52px, 5.4vw, 76px);
		line-height: 0.85;
		text-transform: uppercase;
		background-size: 200% 100%;
		transition: background-position 0.6s ease;
	}
	article:hover h3 {
		background-position: 100% 0;
	}
	p {
		color: var(--text-soft);
	}
	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		font-size: 13px;
		font-weight: 500;
	}
	li {
		border: 1px solid var(--line);
		border-radius: 999px;
		padding: 3px 10px;
		color: var(--text-soft);
	}
	.voet {
		margin-top: auto;
		padding-top: 16px;
		border-top: 1px solid var(--line);
		font-size: 14px;
	}
</style>
