<script lang="ts">
	import JuridischePagina from '#lib/components/JuridischePagina.svelte';
	import { CONTACT_EMAIL } from '#lib/config.js';
</script>

<JuridischePagina
	titel="Privacybeleid"
	pad="/privacy"
	omschrijving="Welke gegevens de Discord-bot Broodkast en deze website verwerken, waarom, en hoe je ze laat verwijderen."
	bijgewerkt="18 november 2026"
>
	<p>
		Broodkast is een Discord-bot en deze website hoort erbij. Beide worden beheerd door <strong>Schellekens Digital</strong>
		(De Kempen, België). Vragen over je gegevens stuur je naar <a href="mailto:{CONTACT_EMAIL}">{CONTACT_EMAIL}</a>.
	</p>

	<h2>Wat de bot bewaart</h2>
	<p>Broodkast bewaart alleen wat nodig is om meldingen te versturen. Per volg, dus per bron die je met <strong>/volg</strong> instelt:</p>
	<ul>
		<li>de ID van de server en van het kanaal waarin de meldingen komen;</li>
		<li>het soort bron (feed, Twitch of webhook) en de bron zelf: de feed-URL, de Twitch-gebruikersnaam of de geheime webhook-token;</li>
		<li>de optionele naam en de optionele rol die je opgaf;</li>
		<li>de Discord-ID van wie de volg aanmaakte, en wanneer;</li>
		<li>wat de bot al gemeld heeft: de ID's van de laatste feed-items of van de laatste Twitch-stream, zodat niets twee keer gepost wordt.</li>
	</ul>
	<p>
		De bot leest geen berichten in je server; hij vraagt Discord daar ook geen toegang toe. Wat je typt bij commando's als
		<strong>/koepraat</strong> gebruikt hij voor het antwoord en bewaart hij niet. Berichten die via een webhook binnenkomen, stuurt hij
		door naar Discord zonder ze op te slaan.
	</p>

	<h2>Waarom</h2>
	<p>
		De gegevens zijn nodig om de dienst te leveren waar een serverbeheerder om vraagt: meldingen in het gekozen kanaal. Dat is de
		rechtsgrond. De Discord-ID van wie een volg aanmaakte, bewaren we om misbruik te kunnen opvolgen.
	</p>

	<h2>Hoe lang</h2>
	<p>Een volg en alles wat erbij hoort, wordt verwijderd zodra:</p>
	<ul>
		<li>iemand hem stopzet met <strong>/ontvolg</strong>;</li>
		<li>het kanaal verwijderd wordt;</li>
		<li>Broodkast uit de server gezet wordt.</li>
	</ul>
	<p>
		Bij fouten schrijft de bot technische logboeken met onder meer server-, kanaal- en volg-ID's en feed-URL's. Die gebruiken we
		alleen om problemen op te lossen.
	</p>

	<h2>Met wie we gegevens delen</h2>
	<ul>
		<li><strong>Discord</strong>, waarop de bot draait en waarin hij meldingen post.</li>
		<li><strong>Twitch</strong>: de bot vraagt aan Twitch of de kanalen die je volgt live zijn.</li>
		<li>De sites achter de <strong>feeds</strong> die je volgt, zoals VRT NWS: de bot haalt die feeds op.</li>
		<li><strong>GamerPower</strong>: daar haalt de bot de lijst met gratis games op. Er gaan geen gegevens van jou naartoe.</li>
		<li><strong>Railway</strong>, waar de bot en zijn database draaien.</li>
		<li><strong>Cloudflare</strong>, dat deze website host.</li>
	</ul>
	<p>We verkopen geen gegevens en gebruiken ze niet voor reclame.</p>

	<h2>Deze website</h2>
	<p>
		De site gebruikt geen cookies en geen bezoekersstatistieken. Cloudflare verwerkt technische gegevens zoals je IP-adres om de site
		te tonen en te beveiligen. De lettertypes komen van Google Fonts; daarvoor maakt je browser verbinding met Google.
	</p>

	<h2>Je rechten</h2>
	<p>
		Je kunt vragen welke gegevens we over jou hebben, ze laten verbeteren of laten verwijderen. Mail naar
		<a href="mailto:{CONTACT_EMAIL}">{CONTACT_EMAIL}</a> met je Discord-ID. Ben je het niet eens met hoe we met je gegevens omgaan, dan
		kun je klacht indienen bij de <a href="https://www.gegevensbeschermingsautoriteit.be">Gegevensbeschermingsautoriteit</a>.
	</p>

	<h2>Wijzigingen</h2>
	<p>Verandert dit beleid, dan passen we de datum bovenaan aan.</p>
</JuridischePagina>
