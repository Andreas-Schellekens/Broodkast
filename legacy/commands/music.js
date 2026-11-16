import { SlashCommandBuilder, EmbedBuilder } from "discord.js";
import { useQueue } from "discord-player";

export default {
    data: new SlashCommandBuilder()
        .setName("muziek")
        .setDescription("Speel muziek af in je spraakkanaal")
        .addSubcommand(sub =>
            sub.setName("speel")
                .setDescription("Speel een nummer of voeg het toe aan de wachtrij")
                .addStringOption(option =>
                    option.setName("nummer")
                        .setDescription("Naam of link (YouTube, Spotify, SoundCloud, ...)")
                        .setRequired(true)
                )
        )
        .addSubcommand(sub =>
            sub.setName("oversla").setDescription("Sla het huidige nummer over")
        )
        .addSubcommand(sub =>
            sub.setName("stop").setDescription("Stop de muziek en verlaat het kanaal")
        )
        .addSubcommand(sub =>
            sub.setName("pauze").setDescription("Pauzeer het huidige nummer")
        )
        .addSubcommand(sub =>
            sub.setName("hervat").setDescription("Hervat het gepauzeerde nummer")
        )
        .addSubcommand(sub =>
            sub.setName("wachtrij").setDescription("Toon de wachtrij")
        )
        .addSubcommand(sub =>
            sub.setName("nu").setDescription("Toon welk nummer nu speelt")
        ),

    async execute(interaction, player) {
        const subcommand = interaction.options.getSubcommand();

        // /muziek speel — the only subcommand that needs to join a channel.
        if (subcommand === "speel") {
            const voiceChannel = interaction.member.voice.channel;
            if (!voiceChannel) {
                return interaction.reply({
                    content: "Je moet eerst in een spraakkanaal zitten!",
                    ephemeral: true
                });
            }

            const query = interaction.options.getString("nummer");
            await interaction.deferReply();

            try {
                const { track } = await player.play(voiceChannel, query, {
                    nodeOptions: {
                        metadata: { channel: interaction.channel },
                        leaveOnEmpty: true,
                        leaveOnEmptyCooldown: 300_000,
                        leaveOnEnd: true,
                        leaveOnEndCooldown: 300_000,
                        volume: 50
                    }
                });

                return interaction.editReply(`🔎 Gevonden en in de wachtrij gezet: **${track.title}**`);
            } catch (error) {
                console.error("Music play error:", error);
                return interaction.editReply(`Kon dit nummer niet afspelen: ${error.message}`);
            }
        }

        // All other subcommands operate on the existing queue for this server.
        const queue = useQueue(interaction.guild);
        if (!queue || !queue.isPlaying()) {
            return interaction.reply({
                content: "Er speelt op dit moment niets.",
                ephemeral: true
            });
        }

        switch (subcommand) {
            case "oversla": {
                queue.node.skip();
                return interaction.reply("⏭️ Nummer overgeslagen.");
            }

            case "stop": {
                queue.delete();
                return interaction.reply("⏹️ Muziek gestopt en kanaal verlaten.");
            }

            case "pauze": {
                queue.node.setPaused(true);
                return interaction.reply("⏸️ Gepauzeerd.");
            }

            case "hervat": {
                queue.node.setPaused(false);
                return interaction.reply("▶️ Hervat.");
            }

            case "nu": {
                const current = queue.currentTrack;
                const embed = new EmbedBuilder()
                    .setTitle("🎶 Nu aan het spelen")
                    .setDescription(`**${current.title}**\n${current.author}`)
                    .setThumbnail(current.thumbnail)
                    .setFooter({ text: `Aangevraagd door ${current.requestedBy?.username ?? "onbekend"}` });
                return interaction.reply({ embeds: [embed] });
            }

            case "wachtrij": {
                const tracks = queue.tracks.toArray();
                if (tracks.length === 0) {
                    return interaction.reply(`Nu speelt: **${queue.currentTrack.title}**\nDe wachtrij is verder leeg.`);
                }

                const upcoming = tracks
                    .slice(0, 10)
                    .map((track, i) => `${i + 1}. ${track.title}`)
                    .join("\n");

                const embed = new EmbedBuilder()
                    .setTitle("📜 Wachtrij")
                    .setDescription(`**Nu speelt:** ${queue.currentTrack.title}\n\n${upcoming}`)
                    .setFooter({ text: tracks.length > 10 ? `en nog ${tracks.length - 10} meer...` : `${tracks.length} in de wachtrij` });
                return interaction.reply({ embeds: [embed] });
            }

            default:
                return interaction.reply({ content: "Onbekend subcommando.", ephemeral: true });
        }
    }
};
