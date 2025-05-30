import { Client, GatewayIntentBits, Collection } from "discord.js";
import { Player } from "discord-player";
import { DefaultExtractors } from "@discord-player/extractor";
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


await client.player.extractors.loadMulti(DefaultExtractors);

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
