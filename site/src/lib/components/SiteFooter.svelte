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
			titel: 'Juridisch',
			links: [
				{ href: '/privacy', label: 'Privacybeleid' },
				{ href: '/voorwaarden', label: 'Gebruiksvoorwaarden' }
			]
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
			<a class="sd" href="https://schellekensdigital.be">
				<span>Gemaakt door</span>
				<img src="/schellekens-digital-creme.svg" alt="Schellekens Digital" width="110" height="30" />
			</a>
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
		align-items: center;
		flex-wrap: wrap;
		font-size: 13px;
	}
	/* In de huisstijl van Schellekens Digital: bosgroen, crème, goud bij hover. */
	.sd {
		display: inline-flex;
		align-items: center;
		gap: 12px;
		padding: 9px 14px;
		background: #16291f;
		border: 1px solid rgb(239 233 220 / 0.18);
		border-radius: 4px;
		color: #efe9dc;
		text-decoration: none;
		font-size: 10px;
		font-weight: 500;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		transition: border-color 0.2s;
	}
	.sd:hover {
		border-color: #c2a36b;
	}
	.sd img {
		display: block;
		height: 26px;
		width: auto;
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
