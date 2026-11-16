import { REST, Routes } from "discord.js";
import { config } from "dotenv";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

config();

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const token = process.env.DISCORD_TOKEN;
const clientId = process.env.CLIENT_ID;
// const guildId = process.env.GUILD_ID; // uit commentaar haal voor testen

const commands = [];

const commandsPath = path.join(__dirname, "commands");
const commandFiles = fs.readdirSync(commandsPath).filter(file => file.endsWith(".js"));

for (const file of commandFiles) {
    const command = await import(`./commands/${file}`);
    console.log("Loading command:", file);

    if (
        command?.default &&
        command.default.data &&
        typeof command.default.data.toJSON === "function"
    ) {
        commands.push(command.default.data.toJSON());
    } else {
        console.warn(`Skipping invalid command file: ${file}`);
        console.log();
    }
}

const rest = new REST({ version: "10" }).setToken(token);

try {
    console.log("Registering global slash commands...");

    console.log();
    // await rest.put(Routes.applicationGuildCommands(clientId , guildId), { body: commands }); // uit commentaar haal voor testen
    await rest.put(
        Routes.applicationCommands(clientId),
        { body: commands }
    );
    console.log("Successfully registered global commands.");
} catch (error) {
    console.error("Failed to register commands:", error);
}
