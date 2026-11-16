<script lang="ts">
	import { INVITE_URL } from '#lib/config.js';
	import Logo from './Logo.svelte';

	const kolommen = [
		{
			titel: 'Bot',
			links: [
				{ href: INVITE_URL, label: 'Uitnodigen' },
				{ href: '/#integraties', label: 'Integraties' },
				{ href: '/commandos', label: "Commando's" }
			]
		},
		{
			titel: 'Meer',
			links: [{ href: 'https://schellekensdigital.be', label: 'schellekensdigital.be' }]
		}
	];
</script>

<footer id="footer">
	<div class="wrap raster">
		<div class="merk">
			<div class="naam"><Logo size={36} /> Broodkast</div>
			<p>De Nederlandstalige Discord-bot die uitzendt wat jij volgt.</p>
		</div>
		{#each kolommen as kolom (kolom.titel)}
			<div class="kolom">
				<span class="eyebrow">{kolom.titel}</span>
				{#each kolom.links as link (link.href)}
					<a href={link.href}>{link.label}</a>
				{/each}
			</div>
		{/each}
		<div class="onder">
			<span>© {new Date().getFullYear()} Broodkast</span>
			<span>broodkast.schellekensdigital.be</span>
		</div>
	</div>
</footer>

<style>
	footer {
		border-top: 1px solid var(--line);
		font-size: 15px;
		color: var(--text-muted);
	}
	.raster {
		display: grid;
		grid-template-columns: minmax(0, 1.6fr) repeat(2, minmax(0, 1fr));
		gap: 32px;
		padding-block: 56px 32px;
	}
	.merk {
		display: flex;
		flex-direction: column;
		gap: 14px;
	}
	.naam {
		display: flex;
		align-items: center;
		gap: 10px;
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 26px;
		text-transform: uppercase;
		color: var(--text);
	}
	.merk p {
		max-width: 34ch;
	}
	.kolom {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}
	.kolom .eyebrow {
		color: var(--text);
	}
	.kolom a {
		text-decoration: none;
	}
	.kolom a:hover {
		color: var(--text);
	}
	.onder {
		grid-column: 1 / -1;
		border-top: 1px solid var(--line);
		padding-top: 20px;
		display: flex;
		justify-content: space-between;
		gap: 12px;
		flex-wrap: wrap;
		font-size: 13px;
	}
	@media (max-width: 760px) {
		.raster {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
		.merk {
			grid-column: 1 / -1;
		}
	}
</style>
