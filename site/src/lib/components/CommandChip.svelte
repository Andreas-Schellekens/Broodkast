<script lang="ts">
	/** Een slash-commando, in het tekstfont (nooit monospace). */
	let { naam }: { naam: string } = $props();
</script>

<span class="chip">{naam}</span>

<style>
	.chip {
		font-family: var(--font-body);
		font-weight: 600;
		color: var(--text);
		white-space: nowrap;
	}
</style>
