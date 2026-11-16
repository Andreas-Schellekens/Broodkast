/**
 * Alle slash-commando's van de bot, voor de pagina /commandos en de homepage.
 * Bron: bot/src/commands/*.rs (de doc comments en #[description]-teksten).
 * Houd dit in sync met de bot; op termijn kan de bot deze lijst als JSON exporteren.
 */

export type Categorie = 'meldingen' | 'leuk' | 'info';

export interface Optie {
	naam: string;
	uitleg: string;
	verplicht?: boolean;
}

export interface Commando {
	naam: string;
	uitleg: string;
	categorie: Categorie;
	/** 'server': alleen in servers. 'overal': ook in DM's en privégroepen. */
	waar: 'server' | 'overal';
	/** Discord-recht dat nodig is om het commando te gebruiken. */
	recht?: string;
	opties?: Optie[];
}

export const CATEGORIEEN: { id: Categorie; naam: string; uitleg: string }[] = [
	{ id: 'meldingen', naam: 'Meldingen', uitleg: 'Bronnen volgen en beheren.' },
	{ id: 'leuk', naam: 'Leuk', uitleg: 'Voor tussendoor.' },
	{ id: 'info', naam: 'Info', uitleg: 'Over de bot zelf.' }
];

const kanaal: Optie = { naam: 'kanaal', uitleg: 'Kanaal voor de meldingen (standaard dit kanaal)' };

export const COMMANDOS: Commando[] = [
	{
		naam: '/volg rss',
		uitleg: 'Volg een RSS- of Atom-feed: YouTube-kanalen, subreddits, GitHub-releases, blogs.',
		categorie: 'meldingen',
		waar: 'server',
		recht: 'Server beheren',
		opties: [
			{ naam: 'url', uitleg: 'URL van de feed', verplicht: true },
			kanaal,
			{ naam: 'rol', uitleg: 'Rol om te pingen bij een nieuw bericht' },
			{ naam: 'naam', uitleg: 'Naam om te tonen (standaard de titel van de feed)' }
		]
	},
	{
		naam: '/volg twitch',
		uitleg: 'Krijg een melding wanneer een Twitch-streamer live gaat.',
		categorie: 'meldingen',
		waar: 'server',
		recht: 'Server beheren',
		opties: [
			{ naam: 'streamer', uitleg: 'Twitch-naam of link naar het kanaal', verplicht: true },
			kanaal,
			{ naam: 'rol', uitleg: 'Rol om te pingen als de stream begint' }
		]
	},
	{
		naam: '/volg webhook',
		uitleg: 'Maak een webhook-URL waarmee andere diensten berichten naar een kanaal sturen.',
		categorie: 'meldingen',
		waar: 'server',
		recht: 'Server beheren',
		opties: [
			{ naam: 'naam', uitleg: 'Naam voor deze webhook, bv. "Server-monitor"', verplicht: true },
			{ naam: 'kanaal', uitleg: 'Kanaal voor de berichten (standaard dit kanaal)' },
			{ naam: 'rol', uitleg: 'Rol om te pingen bij elk bericht' }
		]
	},
	{
		naam: '/volglijst',
		uitleg: 'Toon alles wat deze server volgt.',
		categorie: 'meldingen',
		waar: 'server'
	},
	{
		naam: '/ontvolg',
		uitleg: 'Stop met een abonnement.',
		categorie: 'meldingen',
		waar: 'server',
		recht: 'Server beheren',
		opties: [{ naam: 'abonnement', uitleg: 'Welk abonnement', verplicht: true }]
	},
	{
		naam: '/koepraat',
		uitleg: 'Een koe zegt wat jij typt, in een tekstballon.',
		categorie: 'leuk',
		waar: 'overal',
		opties: [{ naam: 'tekst', uitleg: 'Wat moet onze lieve koe zeggen', verplicht: true }]
	},
	{
		naam: '/muntgooier',
		uitleg: 'Draai een muntje om: kop of munt.',
		categorie: 'leuk',
		waar: 'overal'
	},
	{
		naam: '/apod',
		uitleg: 'De astronomiefoto van de dag van NASA, met uitleg.',
		categorie: 'leuk',
		waar: 'overal'
	},
	{
		naam: '/over',
		uitleg: 'Versie, aantal servers en de uitnodigingslink.',
		categorie: 'info',
		waar: 'overal'
	}
];
