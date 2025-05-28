// 1. Import required modules
const { Client, GatewayIntentBits, Collection } = require("discord.js");
const { Player } = require("discord-player");
const fs = require("fs");
const path = require("path");
require("dotenv").config();

// 2. Create Discord client
const client = new Client({
    intents: [
        GatewayIntentBits.Guilds,
        GatewayIntentBits.GuildVoiceStates
    ]
});

// 3. Attach command collection and player
client.commands = new Collection();
client.player = new Player(client);

// 4. Dynamically load all command files
const commandsPath = path.join(__dirname, "commands");
const commandFiles = fs.readdirSync(commandsPath).filter(file => file.endsWith(".js"));

for (const file of commandFiles) {
    const command = require(`./commands/${file}`);
    client.commands.set(command.data.name, command);
}

// 5. Handle interactions
client.on("interactionCreate", async interaction => {
    if (!interaction.isChatInputCommand()) return;

    const command = client.commands.get(interaction.commandName);
    if (!command) return;

    try {
        await command.execute(interaction, client.player);
    } catch (error) {
        console.error(error);
        await interaction.reply({ content: "❌ There was an error executing this command.", ephemeral: true });
    }
});

// 6. Handle ready event
client.once("ready", () => {
    console.log(`✅ Logged in as ${client.user.tag}`);
});

// 7. Login to Discord
client.login(process.env.DISCORD_TOKEN);
