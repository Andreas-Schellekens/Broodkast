const { SlashCommandBuilder } = require("discord.js");
const { QueryType } = require("discord-player");

module.exports = {
    data: new SlashCommandBuilder()
        .setName("music")
        .setDescription("Music player commands")
        .addSubcommand(sub =>
            sub.setName("play")
                .setDescription("Play a song from YouTube")
                .addStringOption(option =>
                    option.setName("query").setDescription("The song URL or search term").setRequired(true)
                )
        )
        .addSubcommand(sub =>
            sub.setName("skip").setDescription("Skip the current song")
        )
        .addSubcommand(sub =>
            sub.setName("stop").setDescription("Stop playback and clear queue")
        ),

    async execute(interaction, player) {
        const sub = interaction.options.getSubcommand();

        const queue = await player.nodes.create(interaction.guild, {
            metadata: {
                channel: interaction.channel
            },
            leaveOnEmpty: true,
            leaveOnEnd: true,
            leaveOnStop: true
        });

        if (sub === "play") {
            const query = interaction.options.getString("query");

            const searchResult = await player.search(query, {
                requestedBy: interaction.user,
                searchEngine: QueryType.AUTO
            });

            if (!searchResult || !searchResult.tracks.length)
                return interaction.reply("❌ No results found!");

            queue.connect(interaction.member.voice.channel);
            queue.addTrack(searchResult.tracks[0]);

            if (!queue.isPlaying()) await queue.node.play();

            return interaction.reply(`🎶 Now playing: **${searchResult.tracks[0].title}**`);
        }

        if (sub === "skip") {
            if (!queue || !queue.isPlaying()) return interaction.reply("❌ Nothing is playing.");
            await queue.node.skip();
            return interaction.reply("⏭️ Skipped the current song.");
        }

        if (sub === "stop") {
            if (!queue || !queue.isPlaying()) return interaction.reply("❌ Nothing is playing.");
            queue.delete();
            return interaction.reply("🛑 Stopped the music and cleared the queue.");
        }
    }
};
