const { SlashCommandBuilder } = require("discord.js");
const cowsay = require("cowsay");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("koepraat")
        .setDescription("Praat als een koe")
        .addStringOption(option =>
            option.setName("text")
                .setDescription("Wat moet onze lieve koe zeggen")
                .setRequired(true)
        ),

    async execute(interaction) {
        const text = interaction.options.getString("text");

        // Limit input length to prevent spam or formatting issues
        if (text.length > 50) {
            return interaction.reply({
                content: "Das te lang, womp womp minder dan 50 karakters aub",
                ephemeral: true
            });
        }

        const cowOutput = cowsay.say({
            text: text,
            e: "oo",
            T: "U "
        });

        await interaction.reply({
            content: "```\n" + cowOutput + "\n```"
        });
    }
};
