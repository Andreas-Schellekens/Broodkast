<script lang="ts">
	/** Eén voorbeeldmelding zoals Broodkast die in een kanaal zet. */
	let {
		bron,
		titel,
		meta,
		live = false
	}: { bron: string; titel: string; meta?: string; live?: boolean } = $props();
</script>

<div class="kaart" class:live>
	<div class="bron">
		{#if live}<span class="stip" aria-hidden="true"></span>{/if}
		{bron}
	</div>
	<div class="titel">{titel}</div>
	{#if meta}<div class="meta">{meta}</div>{/if}
</div>

<style>
	.kaart {
		background: rgb(11 8 6 / 0.76);
		backdrop-filter: blur(12px);
		border: 1px solid rgb(255 255 255 / 0.16);
		border-radius: 8px;
		padding: 14px 16px;
	}
	.kaart.live {
		border-color: rgb(255 46 136 / 0.6);
	}
	.bron {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--main);
	}
	.live .bron {
		color: var(--accent);
	}
	.stip {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		background: var(--accent);
		box-shadow: 0 0 0 3px rgb(255 46 136 / 0.25);
	}
	.titel {
		font-weight: 600;
		font-size: 16px;
		margin-top: 2px;
	}
	.meta {
		font-size: 13px;
		color: var(--text-muted);
		margin-top: 2px;
	}
	@media (prefers-reduced-motion: no-preference) {
		.stip {
			animation: knipper 1.6s ease-in-out infinite;
		}
		@keyframes knipper {
			50% {
				opacity: 0.35;
			}
		}
	}
</style>
