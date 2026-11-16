# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Broodkast (Dutch pun on "broadcast") is a Dutch-language Discord bot, rebuilt in Rust. Its core feature is integrations: it broadcasts notifications from external services (RSS/Atom feeds, Twitch, inbound webhooks) into Discord channels. Longer-term plans: branding and an invite website.

## Commands

```bash
cargo run                     # start the bot (reads .env, creates ./broodkast.db)
cargo build --release         # optimized build
cargo clippy --all-targets    # lint; keep this warning-free
cargo test                    # unit tests
cargo fmt                     # format
```

There is no separate deploy step for slash commands: the bot registers them on startup. If `DEV_GUILD_ID` is set, commands are registered only in that guild (visible instantly); otherwise globally (can take up to an hour).

## Environment Variables

Read from the environment, with `.env` loaded via `dotenvy` if present. `.env.example` documents all of them.
- `DISCORD_TOKEN`: required
- `DEV_GUILD_ID`: optional test-server ID
- `NASA_API_KEY`: optional, defaults to `DEMO_KEY`
- `DATABASE_URL`: default `sqlite://broodkast.db`; the Docker image uses `/data/broodkast.db`
- `PORT`: web server port, default 8080 (Railway sets it)
- `PUBLIC_URL`: public base URL for webhook links; falls back to `https://$RAILWAY_PUBLIC_DOMAIN`. Without it, `/volg webhook` is disabled.
- `TWITCH_CLIENT_ID` + `TWITCH_CLIENT_SECRET`: both or neither; without them the Twitch integration is off
- `RUST_LOG`: tracing filter, default `info,serenity=warn`

## Architecture

Cargo workspace with one member: `bot/` (crate `broodkast`). Future `site/` and `brand/` folders will sit beside it.

On startup `main.rs` builds an `Arc<App>` (`app.rs`: SQLite pool, a hardened `reqwest::Client`, a standalone `serenity::Http` for posting, optional `TwitchClient`, config), then starts three independent things:
1. **Web server** (`web.rs`, axum): `GET /health` and `POST /webhook/{token}`.
2. **Pollers** (`integrations::spawn_pollers`): one tokio interval task per polling integration (RSS every 10 min, Twitch every 2 min).
3. **Discord gateway** via poise. The poise user data is `Data(Arc<App>)`, which derefs to `App`, so commands use `ctx.data().db` etc.

The poise `event_handler` deletes subscriptions on `GuildDelete` (bot kicked) and `ChannelDelete`. `integrations::broadcast` also deletes subscriptions when Discord answers Unknown Channel.

### Modules
- `db.rs`: all SQL, using `sqlx` runtime queries (not the `query!` macros, so no `DATABASE_URL` is needed at build time). Queries must be static strings; sqlx 0.9 rejects `format!`ed SQL, so use the `select!` macro for column lists. Discord IDs are `u64` but stored as `i64`. Migrations live in `bot/migrations/` and are embedded with `sqlx::migrate!` and run at startup.
- `integrations/`: `mod.rs` holds `spawn_pollers` and `broadcast` (posts an embed with an optional role ping; `allowed_mentions` only allows that role). `rss.rs` and `twitch.rs` each expose `poll(Arc<App>)`.
- `commands/`: one module per command; `mod.rs::all()` is the registered list. `volg.rs` contains `/volg` (subcommands `rss`, `twitch`, `webhook`), `/ontvolg` and `/volglijst`.
- `util.rs`: `truncate`, `strip_html`, `check_public_url` and `fetch_limited`.

### Data model
One table, `subscriptions`: `kind` (`rss`/`twitch`/`webhook`), `target` (feed URL, lowercase Twitch login, or secret webhook token), `channel_id`, optional `ping_role_id` and `label`, and `state`, an integration-specific string:
- RSS: a JSON array of entry IDs from the top 50 items. Anything not in it counts as new. At most 5 items are posted per poll, oldest first. State is set when the subscription is created so existing items are never posted, and it is updated even after a failed send to avoid duplicates.
- Twitch: the ID of the last announced stream. It is deliberately not cleared when a channel goes offline, so API blips don't cause duplicate announcements.

### Security
User-supplied URLs go through `check_public_url` (http/https only; no localhost, private or link-local IPs, or `*.internal`/`*.local` hosts, which blocks Railway's private network). The redirect policy in `main.rs::http_client` applies the same check to every redirect. Responses are size-capped. DNS names that resolve to private IPs are not caught. Webhook tokens are 40 random alphanumeric characters, and webhooks are rate-limited to 1 message per 2 s per token, in memory.

### Adding a Command
1. Create `bot/src/commands/<naam>.rs` with `#[poise::command(slash_command, ...)] pub async fn <naam>(ctx: Context<'_>, ...) -> Result<(), Error>`. The doc comment becomes the Discord description, and `#[description = "..."]` documents parameters.
2. Add it to `commands/mod.rs::all()`.
3. Contexts: fun or stateless commands get `install_context = "Guild|User"` and `interaction_context = "Guild|BotDm|PrivateChannel"` (user-installable). Commands that change server settings use `guild_only`, `install_context = "Guild"`, and `default_member_permissions` plus `required_permissions = "MANAGE_GUILD"`.

### Adding an Integration
Add a `Kind` variant in `db.rs`, plus a new migration that updates the `CHECK` constraint (SQLite needs a table rebuild for that). Write `integrations/<naam>.rs` with a `poll` fn, register it in `spawn_pollers`, and add a subcommand to `/volg`. Push-based integrations get a route in `web.rs` instead of a poller.

### Naming Convention
Commands, parameters and user-facing text are in Dutch (e.g. `muntgooier`, `volglijst`). Code comments can be Dutch or English. Brand colour: `BROODKAST_KLEUR` (0xC8873A).

## Deployment (Railway)
`Dockerfile` (multi-stage: `rust:1-slim-trixie` → `debian:trixie-slim`) plus `railway.toml` (health check `/health`, 1 replica). The SQLite database must be on a Railway volume mounted at `/data`. The container runs as root on purpose: Railway mounts volumes root-owned, and a non-root user gets "unable to open database file". Run only one instance, because of SQLite and the single gateway connection.

## Music (not yet ported)
The old `/muziek` (in `legacy/`) streamed YouTube via yt-dlp. The planned Rust stack is `songbird`; the `GUILD_VOICE_STATES` intent is already enabled. Streaming YouTube from a public bot breaks YouTube's terms and has gotten large music bots shut down, so decide on sources first.

## Legacy
`legacy/` holds the original discord.js bot for reference only. It is not built or run. Don't modify it.
