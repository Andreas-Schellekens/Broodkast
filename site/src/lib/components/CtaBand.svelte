<script lang="ts">
	import InviteButton from './InviteButton.svelte';
</script>

<section>
	<img src="/thermal.webp" alt="" aria-hidden="true" width="1800" height="1200" loading="lazy" />
	<div class="wrap inhoud">
		<h2 class="display">Zet de broodkast in je server</h2>
		<p>Je voegt de bot toe via Discord. Daarna volg je je eerste bron met /volg.</p>
		<InviteButton size="lg" />
	</div>
</section>

<style>
	section {
		position: relative;
		overflow: hidden;
		border-top: 1px solid var(--line);
	}
	img {
		position: absolute;
		left: 0;
		top: 0;
		width: 70%;
		height: 100%;
		object-fit: cover;
		object-position: left bottom;
		pointer-events: none;
	}
	/* Rechts zwart, zodat de roze knop op donker staat en knalt. */
	section::before {
		content: '';
		position: absolute;
		inset: 0;
		z-index: 1;
		background: linear-gradient(270deg, var(--bg) 0%, var(--bg) 34%, rgb(11 8 6 / 0) 62%);
		pointer-events: none;
	}
	.inhoud {
		position: relative;
		z-index: 2;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		text-align: right;
		gap: 22px;
		padding-block: 140px;
	}
	h2 {
		font-size: clamp(48px, 6vw, 92px);
		line-height: 0.88;
		max-width: 11ch;
		text-shadow: 0 2px 30px rgb(11 8 6 / 0.35);
	}
	p {
		max-width: 34ch;
		font-weight: 500;
		background: rgb(11 8 6 / 0.7);
		padding: 6px 10px;
		border-radius: var(--radius-sm);
	}
	@media (max-width: 900px) {
		.inhoud {
			align-items: flex-start;
			text-align: left;
			padding-block: 96px;
		}
		img {
			width: 100%;
			opacity: 0.55;
		}
		section::before {
			background: rgb(11 8 6 / 0.35);
		}
	}
</style>
