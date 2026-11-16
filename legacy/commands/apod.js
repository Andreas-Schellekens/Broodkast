import { EmbedBuilder, SlashCommandBuilder, AttachmentBuilder } from "discord.js";
import fetch from "node-fetch";
import sharp from "sharp";
import dotenv from "dotenv";
import fs from "fs/promises";
import path from "path";

// Load environment variables
dotenv.config();

const url = `https://api.nasa.gov/planetary/apod?api_key=${process.env.NASA_API_KEY}`;

export default {
    data: new SlashCommandBuilder()
        .setName("apod")
        .setDescription("Astronomy Picture of the Day"),

    async execute(interaction) {
        try {
            const response = await fetch(url);
            const data = await response.json();

            if (data.media_type !== "image") {
                const embed = new EmbedBuilder()
                    .setTitle(data.title)
                    .setDescription(`${data.explanation}\n\n[Watch Video](${data.url})`)
                    .setFooter({ text: `© NASA | ${data.date}` });

                return await interaction.reply({ embeds: [embed] });
            }

            const imageUrl = data.hdurl || data.url;


            const imageResponse = await fetch(imageUrl);
            const buffer = await imageResponse.buffer();


            const outputPath = path.resolve("cropped.jpg");
            await sharp(buffer)
                .resize(500, 500)
                .toFile(outputPath);


            const attachment = new AttachmentBuilder(outputPath, { name: "apod.jpg" });


            const embed = new EmbedBuilder()
                .setTitle(data.title)
                .setDescription(data.explanation)
                .setImage("attachment://apod.jpg")
                .setURL(imageUrl)
                .setFooter({
                    text: `© NASA | ${data.date}`,
                    iconURL: 'https://scitechdaily.com/images/NASA-Meatball-Logo.png'
                });

            await interaction.reply({ embeds: [embed], files: [attachment] });


            await fs.unlink(outputPath);
        } catch (error) {
            console.error(error);
            await interaction.reply({
                content: "Fout bij het ophalen of verwerken van de foto."
            });
        }
    }
};
