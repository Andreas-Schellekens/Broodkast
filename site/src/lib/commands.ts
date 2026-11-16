/**
 * Alle slash-commando's van de bot, voor de pagina /commandos en de homepage.
 * Bron: bot/src/commands/*.rs (de doc comments en #[description]-teksten).
 * Houd dit in sync met de bot; op termijn kan de bot deze lijst als JSON exporteren.
 */

export type Categorie = 'meldingen' | 'handig' | 'leuk';

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
	{ id: 'handig', naam: 'Handig', uitleg: 'Hulpjes voor jou en je server.' },
	{ id: 'leuk', naam: 'Leuk', uitleg: 'Voor tussendoor.' }
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
		waar: 'server',
		recht: 'Server beheren'
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
		naam: '/herinner nieuw',
		uitleg: 'Laat Broodkast je later iets herinneren. Zit de bot niet in de server, dan komt de herinnering via DM.',
		categorie: 'handig',
		waar: 'overal',
		opties: [
			{ naam: 'over', uitleg: 'Wanneer, bv. 10m, 2u, 1d of 1u30m', verplicht: true },
			{ naam: 'wat', uitleg: 'Waaraan je herinnerd wilt worden', verplicht: true }
		]
	},
	{
		naam: '/herinner lijst',
		uitleg: 'Toon je herinneringen die nog moeten komen.',
		categorie: 'handig',
		waar: 'overal'
	},
	{
		naam: '/herinner wis',
		uitleg: 'Verwijder een herinnering.',
		categorie: 'handig',
		waar: 'overal'
	},
	{
		naam: '/weer',
		uitleg: 'Het weer van nu en de komende dagen.',
		categorie: 'handig',
		waar: 'overal',
		opties: [{ naam: 'plaats', uitleg: 'Gemeente of stad', verplicht: true }]
	},
	{
		naam: '/gebruiker',
		uitleg: 'Info en profielfoto van iemand.',
		categorie: 'handig',
		waar: 'overal',
		opties: [{ naam: 'wie', uitleg: 'Over wie (standaard jijzelf)' }]
	},
	{
		naam: '/serverinfo',
		uitleg: 'Info over deze server.',
		categorie: 'handig',
		waar: 'server'
	},
	{
		naam: '/opruimen',
		uitleg: 'Verwijder de laatste berichten in dit kanaal.',
		categorie: 'handig',
		waar: 'server',
		recht: 'Berichten beheren',
		opties: [
			{ naam: 'aantal', uitleg: 'Hoeveel berichten', verplicht: true },
			{ naam: 'van', uitleg: 'Alleen berichten van deze persoon' }
		]
	},
	{
		naam: '/apod',
		uitleg: 'De astronomiefoto van de dag van NASA, met uitleg.',
		categorie: 'handig',
		waar: 'overal'
	},
	{
		naam: '/over',
		uitleg: 'Versie, aantal servers en de uitnodigingslink.',
		categorie: 'handig',
		waar: 'overal'
	},
	{
		naam: '/muntgooier',
		uitleg: 'Draai een muntje om: kop of munt.',
		categorie: 'leuk',
		waar: 'overal'
	},
	{
		naam: '/dobbel',
		uitleg: 'Gooi dobbelstenen.',
		categorie: 'leuk',
		waar: 'overal',
		opties: [{ naam: 'worp', uitleg: 'Welke worp, bv. 2d6 of d20+3' }]
	},
	{
		naam: '/kies',
		uitleg: 'Broodkast kiest voor jou.',
		categorie: 'leuk',
		waar: 'overal',
		opties: [{ naam: 'opties', uitleg: 'Waartussen, bv. "pizza, frietjes of sushi"', verplicht: true }]
	},
	{
		naam: '/glazenbol',
		uitleg: 'Stel een ja-neevraag; de glazen bol weet het.',
		categorie: 'leuk',
		waar: 'overal',
		opties: [{ naam: 'vraag', uitleg: 'Je vraag', verplicht: true }]
	},
	{
		naam: '/mop',
		uitleg: 'Een mop, met de clou achter een spoiler.',
		categorie: 'leuk',
		waar: 'overal'
	},
	{
		naam: '/koepraat',
		uitleg: 'Een koe zegt wat jij typt, in een tekstballon.',
		categorie: 'leuk',
		waar: 'overal',
		opties: [{ naam: 'tekst', uitleg: 'Wat moet onze lieve koe zeggen', verplicht: true }]
	}
];
