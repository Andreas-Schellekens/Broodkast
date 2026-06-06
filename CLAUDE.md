# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Install dependencies. The env var is REQUIRED: youtube-dl-exec's preinstall
# script aborts if it can't find Python, even though the bundled yt-dlp binary
# does not need Python at runtime on Windows.
YOUTUBE_DL_SKIP_PYTHON_CHECK=1 npm install   # PowerShell: $env:YOUTUBE_DL_SKIP_PYTHON_CHECK=1; npm install

# Start the bot
node index.js

# Start with auto-reload on changes
npx nodemon index.js

# Register slash commands globally (takes ~1 hour to propagate)
node deploy-commands.js
```

To register commands instantly for testing on a specific guild, uncomment the `guildId` lines in `deploy-commands.js` (both the variable declaration and the `applicationGuildCommands` call).

## Environment Variables

A `.env` file is required with:
- `DISCORD_TOKEN` — bot token from Discord Developer Portal
- `CLIENT_ID` — application/bot ID
- `GUILD_ID` — test server ID (only needed for guild-scoped command deployment)
- `NASA_API_KEY` — from api.nasa.gov (use `DEMO_KEY` for light testing)

`config.json` (also gitignored) mirrors these values but is used only as a reference — the bot reads exclusively from `.env`.

## Architecture

`index.js` is the entry point. It creates a Discord `Client`, attaches a `discord-player` `Player` instance to it, and auto-loads every `.js` file from `commands/` into `client.commands`. On `interactionCreate` it dispatches to the matching command's `execute(interaction, player)` method.

### Music / YouTube playback

discord-player v7's `DefaultExtractors` **no longer ship a working YouTube extractor**, and the Spotify/Apple Music extractors only search — they bridge to YouTube for the actual audio. So `index.js` registers `YoutubeiExtractor` (from `discord-player-youtubei`) **before** `loadMulti(DefaultExtractors)`, configured with:
- `useYoutubeDL: true` — streams via the bundled **yt-dlp** binary instead of `youtubei.js`. `youtubei.js` regularly breaks with `Failed to extract signature decipher function` warnings whenever YouTube updates its player; yt-dlp is maintained against those changes and is the reliable stream source. (The decipher warnings still print at startup but are harmless because yt-dlp does the streaming.)
- `generateWithPoToken: true` — mints a YouTube poToken via bundled `bgutils-js` so search/metadata requests are less likely to be bot-flagged.

`index.js` also attaches `client.player.events` listeners (`playerStart`, `audioTrackAdd`, `emptyQueue`, `disconnect`, `error`, `playerError`) that post status messages to the text channel. The channel is passed through `queue.metadata.channel`, which the `/muziek speel` handler sets via `nodeOptions.metadata`.

The music command itself (`commands/music.js`) is a single `/muziek` slash command with subcommands (`speel`, `oversla`, `stop`, `pauze`, `hervat`, `wachtrij`, `nu`) rather than separate files — `speel` joins the caller's voice channel and uses `player.play()`; the rest operate on the existing queue via `useQueue(interaction.guild)`.

### Adding a Command

Create `commands/<name>.js` exporting a default object with two properties:
- `data` — a `SlashCommandBuilder` instance (defines the slash command name, description, options)
- `execute(interaction, player)` — async handler; `player` is the `discord-player` Player

The file is picked up automatically on next bot start — no registration in a manifest needed. Run `deploy-commands.js` after adding or changing commands so Discord shows the updated definitions.

### Command Naming Convention

Commands use Dutch names (e.g. `muntgooier`, `koepraat`). Keep new commands consistent with this.

## Key Dependencies

| Package | Purpose |
|---|---|
| `discord.js` v14 | Core Discord API client |
| `discord-player` + `@discord-player/extractor` | Audio playback in voice channels |
| `discord-player-youtubei` | YouTube search + streaming extractor for discord-player v7 |
| `youtube-dl-exec` | Bundles the yt-dlp binary used by the extractor's `useYoutubeDL` stream engine |
| `@discordjs/voice` + `@discordjs/opus` | Voice connection layer for discord-player |
| `ffmpeg-static` | Provides the ffmpeg binary discord-player uses to transcode audio (auto-detected by prism-media) |
| `sharp` | Image resizing (used in `apod.js`) |
| `node-fetch` | HTTP requests to external APIs |
| `cowsay` | ASCII cow art for `koepraat` command |

`ytdl-core` is still listed in `package.json` but is unused — discord-player v7 does not use it.

## Known Issues

- `/apod` is currently broken (`apod.js` uses `imageResponse.buffer()`, removed in `node-fetch` v3 — use `arrayBuffer()` instead). Intentionally left unfixed for now.

## Project Type

ESM module (`"type": "module"` in `package.json`). Use `import`/`export` throughout; `require()` will not work.
