import {SlashCommandBuilder} from "discord.js";

export default {
    data: new SlashCommandBuilder()
        .setName("muntgooier")
        .setDescription("Draai een muntje om"),


    async execute(interaction) {


        const randomNumber = Math.floor(Math.random() * 2) + 1;

        function coinFlipper(randomNumber) {

            if (randomNumber === 1) {
                return "kop";
            }
            return "munt";

        }

        await interaction.reply({
            content: coinFlipper(randomNumber)
        });
    }
};