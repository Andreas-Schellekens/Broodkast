<script lang="ts">
	import CommandChip from './CommandChip.svelte';

	let {
		label,
		waarde,
		titel,
		tekst,
		commando,
		limiet
	}: {
		label: string;
		waarde: string;
		titel: string;
		tekst: string;
		commando: string;
		limiet: string;
	} = $props();
</script>

<article>
	<div class="eyebrow">{label}</div>
	<div class="waarde hot">{waarde}</div>
	<h3>{titel}</h3>
	<p>{tekst}</p>
	<div class="voet"><CommandChip naam={commando} /><span>{limiet}</span></div>
</article>

<style>
	article {
		background: var(--surface);
		padding: 32px;
		display: flex;
		flex-direction: column;
		gap: 14px;
		min-width: 0;
	}
	.waarde {
		font-family: var(--font-display);
		font-weight: 900;
		font-size: 84px;
		line-height: 0.85;
		font-variant-numeric: tabular-nums;
		text-transform: uppercase;
	}
	h3 {
		margin-top: 8px;
		font-size: 22px;
		font-weight: 600;
	}
	p {
		color: var(--text-soft);
	}
	.voet {
		margin-top: auto;
		padding-top: 16px;
		border-top: 1px solid var(--line);
		display: flex;
		justify-content: space-between;
		gap: 12px;
		flex-wrap: wrap;
		font-size: 14px;
		color: var(--text-muted);
	}
</style>
