# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Broodkast (Dutch pun on "broadcast") is a Dutch-language Discord bot, rebuilt in Rust. Longer-term plans: branding and an invite website.

## Commands

```bash
cargo run                     # start the bot (reads .env)
cargo build --release         # optimized build
cargo clippy --all-targets    # lint; keep this warning-free
cargo test                    # unit tests
cargo fmt                     # format
```

There is no separate deploy step: the bot registers its slash commands on startup. If `DEV_GUILD_ID` is set, commands are registered only in that guild (visible instantly); otherwise they are registered globally (can take up to an hour).

## Environment Variables

Read from the environment, with `.env` loaded via `dotenvy` if present. See `.env.example`.
- `DISCORD_TOKEN`: required
- `DEV_GUILD_ID`: optional test-server ID (replaces the old `GUILD_ID`)
- `NASA_API_KEY`: optional, defaults to `DEMO_KEY`
- `RUST_LOG`: optional tracing filter, default `info,serenity=warn`

## Architecture

Cargo workspace (`Cargo.toml` at the root) with one member so far: `bot/` (crate `broodkast`). Future `site/` and `brand/` folders will sit beside it.

- `bot/src/main.rs`: sets up tracing, loads `Config`, builds the `poise` framework, registers commands in `setup`, and starts the serenity client (autosharded, Ctrl+C shuts down cleanly). It also defines the shared `Data` struct (a reused `reqwest::Client` plus config values), the `Context`/`Error` aliases (`anyhow::Error`), and a global `on_error` that logs command failures and sends the user an ephemeral Dutch error message.
- `bot/src/config.rs`: env parsing.
- `bot/src/commands/`: one module per command. `mod.rs::all()` is the list of registered commands.

Use `poise::serenity_prelude as serenity` instead of depending on `serenity` directly, so the version always matches poise (poise 0.7 → serenity 0.12).

### Adding a Command

1. Create `bot/src/commands/<naam>.rs` with a `#[poise::command(slash_command)] pub async fn <naam>(ctx: Context<'_>, ...) -> Result<(), Error>`. The doc comment becomes the Discord description, and `#[description = "..."]` documents parameters.
2. Add `mod <naam>;` and `<naam>::<naam>()` to `commands/mod.rs`.

### Naming Convention

Commands, parameters and user-facing text are in Dutch (e.g. `muntgooier`, `koepraat`). Code comments can be Dutch or English.

## Music (not yet ported)

The old `/muziek` command (in `legacy/`) streamed YouTube through yt-dlp. In Rust the planned stack is `songbird` (already reachable through the `GUILD_VOICE_STATES` intent). Streaming YouTube from a public bot breaks YouTube's terms and has gotten large music bots shut down, so decide on sources (own servers only, SoundCloud or radio streams, etc.) before building it.

## Legacy

`legacy/` holds the original discord.js bot for reference only. It is not built or run. Don't modify it, except to delete it once everything is ported.
