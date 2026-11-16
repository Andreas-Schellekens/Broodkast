<script lang="ts">
	import Logo from './Logo.svelte';
	import InviteButton from './InviteButton.svelte';

	const links = [
		{ href: '/#integraties', label: 'Integraties' },
		{ href: '/commandos', label: "Commando's" },
		{ href: '#footer', label: 'Contact' }
	];
</script>

<header>
	<div class="wrap balk">
		<a class="merk" href="/" aria-label="Broodkast, naar de homepage">
			<Logo size={44} />
			<span>Broodkast</span>
		</a>
		<nav aria-label="Hoofdmenu">
			{#each links as link (link.href)}
				<a href={link.href}>{link.label}</a>
			{/each}
		</nav>
		<InviteButton size="sm" />
	</div>
</header>

<style>
	header {
		position: relative;
		z-index: 2;
	}
	.balk {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		height: 84px;
	}
	.merk {
		display: flex;
		align-items: center;
		gap: 10px;
		text-decoration: none;
		font-family: var(--font-display);
		font-weight: 800;
		font-size: 28px;
		text-transform: uppercase;
		letter-spacing: 0.01em;
	}
	nav {
		display: flex;
		gap: 32px;
		font-size: 15px;
		font-weight: 500;
	}
	nav a {
		text-decoration: none;
		color: var(--text-soft);
	}
	nav a:hover {
		color: var(--text);
	}
	@media (max-width: 900px) {
		nav {
			display: none;
		}
	}
	@media (max-width: 520px) {
		.merk span {
			font-size: 22px;
		}
	}
</style>
