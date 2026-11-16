<script lang="ts">
	import JuridischePagina from '#lib/components/JuridischePagina.svelte';
	import { CONTACT_EMAIL } from '#lib/config.js';
</script>

<JuridischePagina
	titel="Gebruiksvoorwaarden"
	pad="/voorwaarden"
	omschrijving="De voorwaarden om de Discord-bot Broodkast te gebruiken."
	bijgewerkt="16 november 2026"
>
	<p>
		Door Broodkast aan een Discord-server toe te voegen of zijn commando's te gebruiken, ga je akkoord met deze voorwaarden. Broodkast
		wordt beheerd door <strong>Schellekens Digital</strong> (De Kempen, België). Hoe we met gegevens omgaan, staat in het
		<a href="/privacy">privacybeleid</a>.
	</p>

	<h2>De dienst</h2>
	<p>
		Broodkast is gratis. Hij post meldingen van feeds, Twitch en webhooks in Discord-kanalen en heeft een paar commando's voor de lol.
		We kunnen functies toevoegen, wijzigen of stopzetten, en de bot tijdelijk offline halen voor onderhoud.
	</p>

	<h2>Wat we van jou verwachten</h2>
	<ul>
		<li>Je volgt de <a href="https://discord.com/terms">voorwaarden</a> en <a href="https://discord.com/guidelines">richtlijnen</a> van Discord.</li>
		<li>
			Je bent verantwoordelijk voor de bronnen die je koppelt en voor wat er via je webhook binnenkomt. Gebruik Broodkast niet om
			spam, illegale inhoud of inhoud waarvoor je geen rechten hebt te verspreiden.
		</li>
		<li>Je houdt de link van je webhook geheim. Wie de link heeft, kan in je kanaal posten.</li>
		<li>Je probeert de bot of de servers erachter niet te overbelasten, te misbruiken of te omzeilen.</li>
	</ul>

	<h2>Misbruik</h2>
	<p>
		Bij misbruik kunnen we volgs verwijderen, webhooks uitschakelen of een server de toegang tot Broodkast ontzeggen, zonder
		voorafgaande verwittiging.
	</p>

	<h2>Geen garanties</h2>
	<p>
		We doen ons best om Broodkast goed te laten werken, maar leveren hem zoals hij is. Een melding kan te laat komen of ontbreken,
		bijvoorbeeld als een feed of Twitch niet bereikbaar is. Voor zover de wet het toelaat, zijn we niet aansprakelijk voor schade door
		het gebruik van Broodkast of doordat hij niet werkt.
	</p>

	<h2>Stoppen</h2>
	<p>
		Je kunt op elk moment stoppen door Broodkast uit je server te zetten. Zijn volgs voor die server worden dan verwijderd.
	</p>

	<h2>Wijzigingen</h2>
	<p>
		We kunnen deze voorwaarden aanpassen. De datum bovenaan toont de laatste versie. Wie Broodkast daarna blijft gebruiken, aanvaardt
		de nieuwe versie.
	</p>

	<h2>Recht en contact</h2>
	<p>
		Op deze voorwaarden is het Belgische recht van toepassing. Vragen? Mail naar <a href="mailto:{CONTACT_EMAIL}">{CONTACT_EMAIL}</a>.
	</p>
</JuridischePagina>
