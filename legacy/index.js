import { Client, GatewayIntentBits, Collection } from "discord.js";
import { Player } from "discord-player";
import { DefaultExtractors } from "@discord-player/extractor";
import { YoutubeiExtractor } from "discord-player-youtubei";
import fs from "fs";
import path from "path";
import dotenv from "dotenv";
import { fileURLToPath } from "url";

dotenv.config();

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const client = new Client({
    intents: [
        GatewayIntentBits.Guilds,
        GatewayIntentBits.GuildVoiceStates
    ]
});

client.commands = new Collection();
client.player = new Player(client);

// YouTube streaming/search. discord-player v7's DefaultExtractors no longer ship a
// working YouTube extractor, and Spotify/Apple Music searches bridge to YouTube for
// the actual audio, so this must be registered first.
//   - useYoutubeDL resolves the actual audio stream via the bundled yt-dlp binary,
//     which is maintained against YouTube's changes — it avoids youtubei.js's
//     "signature decipher" breakage that recurs whenever YouTube updates its player.
//   - generateWithPoToken mints a YouTube "poToken" (via bundled bgutils-js) so
//     search/metadata requests are less likely to be flagged as a bot.
await client.player.extractors.register(YoutubeiExtractor, {
    useYoutubeDL: true,
    generateWithPoToken: true
});
await client.player.extractors.loadMulti(DefaultExtractors);

// Announce playback state in the text channel the command was used in.
// queue.metadata.channel is set by the /muziek play handler.
client.player.events.on("playerStart", (queue, track) => {
    queue.metadata?.channel?.send(`🎶 Nu aan het spelen: **${track.title}**`).catch(() => {});
});

client.player.events.on("audioTrackAdd", (queue, track) => {
    queue.metadata?.channel?.send(`➕ Toegevoegd aan de wachtrij: **${track.title}**`).catch(() => {});
});

client.player.events.on("emptyQueue", (queue) => {
    queue.metadata?.channel?.send("✅ De wachtrij is leeg, tot de volgende keer!").catch(() => {});
});

client.player.events.on("disconnect", (queue) => {
    queue.metadata?.channel?.send("👋 Ik ben uit het spraakkanaal gegaan.").catch(() => {});
});

client.player.events.on("error", (queue, error) => {
    console.error(`Player queue error: ${error.message}`);
});

client.player.events.on("playerError", (queue, error) => {
    console.error(`Player stream error: ${error.message}`);
    queue.metadata?.channel?.send("⚠️ Er ging iets mis tijdens het afspelen.").catch(() => {});
});

const commandsPath = path.join(__dirname, "commands");
const commandFiles = fs.readdirSync(commandsPath).filter(file => file.endsWith(".js"));

for (const file of commandFiles) {
    const command = await import(`./commands/${file}`);
    const cmd = command.default;

    if (!cmd || !cmd.data || typeof cmd.data.name !== "string") {
        console.warn(`Skipping invalid command file: ${file}`);
        continue;
    }

    client.commands.set(cmd.data.name, cmd);
}


client.on("interactionCreate", async interaction => {
    if (!interaction.isChatInputCommand()) return;

    const command = client.commands.get(interaction.commandName);
    if (!command) return;

    try {
        await command.execute(interaction, client.player);
    } catch (error) {
        console.error("Command execution error:", error);
        await interaction.reply({
            content: "There was an error executing this command.",
            ephemeral: true
        });
    }
});

client.once("ready", () => {
    console.log(`Logged in as ${client.user.tag}`);
    console.log();
    console.log("It's showtime")
});

client.login(process.env.DISCORD_TOKEN);
