import { SlashCommandBuilder } from "discord.js";
import cowsay from "cowsay";

export default {
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


        if (text.length > 50) {
            return interaction.reply({
                content: "Das te lang, womp womp minder dan 50 karakters aub",
                ephemeral: true
            });
        }

        const cowOutput = cowsay.say({
            text: text,
            e: "oo"
        });

        await interaction.reply({
            content: "```\n" + cowOutput + "\n```"
        });
    }
};
