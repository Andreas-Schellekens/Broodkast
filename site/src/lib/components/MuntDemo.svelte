<script lang="ts">
	let kop = $state(true);
	let draai = $state(0);

	function gooi() {
		kop = Math.random() < 0.5;
		draai += 720;
	}
</script>

<div class="rij">
	<button type="button" onclick={gooi} aria-label="Gooi het muntje" style:transform="rotateY({draai}deg)">
		{kop ? 'K' : 'M'}
	</button>
	<div>
		<div class="uitkomst" aria-live="polite">{kop ? 'Kop!' : 'Munt!'}</div>
		<div class="hint">Tik op het muntje.</div>
	</div>
</div>

<style>
	.rij {
		display: flex;
		align-items: center;
		gap: 14px;
	}
	button {
		flex: none;
		width: 52px;
		height: 52px;
		border-radius: 50%;
		border: 0;
		cursor: pointer;
		background: radial-gradient(circle at 35% 35%, #ffd45a, var(--main) 60%, var(--accent));
		color: var(--secondary);
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 24px;
		transition: transform 0.6s cubic-bezier(0.3, 1.4, 0.5, 1);
	}
	@media (prefers-reduced-motion: reduce) {
		button {
			transition: none;
		}
	}
	.uitkomst {
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 28px;
		line-height: 1;
		text-transform: uppercase;
	}
	.hint {
		font-size: 14px;
		color: var(--text-muted);
		margin-top: 4px;
	}
</style>
