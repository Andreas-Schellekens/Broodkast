<script lang="ts">
	let invoer = $state('');
	let ballon = $state('Boe. Er is een nieuwe video.');

	function zeg(e: SubmitEvent) {
		e.preventDefault();
		ballon = invoer.trim() || 'Boe!';
	}
</script>

<div class="ballon" aria-live="polite">{ballon}</div>
<form onsubmit={zeg}>
	<label class="sr-only" for="koetekst">Wat moet de koe zeggen?</label>
	<input id="koetekst" bind:value={invoer} maxlength="80" autocomplete="off" placeholder="Wat moet de koe zeggen?" />
	<button type="submit">Zeg het</button>
</form>

<style>
	.ballon {
		display: inline-block;
		max-width: 100%;
		background: var(--text);
		color: var(--secondary);
		border-radius: 12px 12px 12px 3px;
		padding: 10px 14px;
		font-weight: 600;
		overflow-wrap: anywhere;
	}
	form {
		display: flex;
		gap: 8px;
		margin-top: 14px;
		flex-wrap: wrap;
	}
	input {
		flex: 1;
		min-width: 0;
		font: 500 15px var(--font-body);
		color: var(--text);
		background: rgb(255 255 255 / 0.06);
		border: 1px solid var(--line-strong);
		border-radius: var(--radius-sm);
		padding: 11px 12px;
	}
	input::placeholder {
		color: var(--text-muted);
	}
	button {
		font: 600 15px var(--font-body);
		color: var(--text);
		background: none;
		border: 1px solid var(--line-strong);
		border-radius: var(--radius-sm);
		padding: 11px 14px;
		cursor: pointer;
	}
	button:hover {
		border-color: var(--text);
	}
</style>
