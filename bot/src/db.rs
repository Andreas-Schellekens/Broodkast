//! Opslag in SQLite. Discord-ID's zijn u64; SQLite kent alleen i64, dus we casten bij de grens.

use std::{fmt, str::FromStr};

use anyhow::Context as _;
use poise::serenity_prelude::{ChannelId, GuildId, RoleId, UserId};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

pub async fn connect(url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(url)
        .with_context(|| format!("ongeldige DATABASE_URL: {url}"))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .context("kon database niet openen")?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("databasemigraties mislukt")?;
    Ok(pool)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Rss,
    Twitch,
    Webhook,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Rss => "rss",
            Kind::Twitch => "twitch",
            Kind::Webhook => "webhook",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            Kind::Rss => "📰",
            Kind::Twitch => "🟣",
            Kind::Webhook => "🪝",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Kind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "rss" => Ok(Kind::Rss),
            "twitch" => Ok(Kind::Twitch),
            "webhook" => Ok(Kind::Webhook),
            other => anyhow::bail!("onbekend soort abonnement: {other}"),
        }
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    id: i64,
    guild_id: i64,
    channel_id: i64,
    kind: String,
    target: String,
    label: Option<String>,
    ping_role_id: Option<i64>,
    state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: i64,
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub kind: Kind,
    pub target: String,
    pub label: Option<String>,
    pub ping_role_id: Option<RoleId>,
    pub state: Option<String>,
}

impl Subscription {
    /// Naam om in berichten te tonen: het label, anders het doel (behalve bij geheime webhooks).
    pub fn display_name(&self) -> &str {
        match (&self.label, self.kind) {
            (Some(l), _) => l,
            (None, Kind::Webhook) => "webhook",
            (None, _) => &self.target,
        }
    }
}

impl TryFrom<Row> for Subscription {
    type Error = anyhow::Error;
    fn try_from(r: Row) -> anyhow::Result<Self> {
        Ok(Self {
            id: r.id,
            guild_id: GuildId::new(r.guild_id as u64),
            channel_id: ChannelId::new(r.channel_id as u64),
            kind: r.kind.parse()?,
            target: r.target,
            label: r.label,
            ping_role_id: r.ping_role_id.map(|id| RoleId::new(id as u64)),
            state: r.state,
        })
    }
}

fn convert(rows: Vec<Row>) -> anyhow::Result<Vec<Subscription>> {
    rows.into_iter().map(Subscription::try_from).collect()
}

/// Statische SELECT-query; `concat!` houdt het een `&'static str`, dus geen dynamische SQL.
macro_rules! select {
    ($rest:literal) => {
        concat!(
            "SELECT id, guild_id, channel_id, kind, target, label, ping_role_id, state ",
            "FROM subscriptions ",
            $rest
        )
    };
}

pub struct NewSubscription<'a> {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub kind: Kind,
    pub target: &'a str,
    pub label: Option<&'a str>,
    pub ping_role_id: Option<RoleId>,
    pub state: Option<&'a str>,
    pub created_by: UserId,
}

/// Uitkomst van [`insert`]: `None` als exact deze combinatie al bestaat.
pub async fn insert(db: &SqlitePool, new: NewSubscription<'_>) -> anyhow::Result<Option<i64>> {
    let result = sqlx::query(
        "INSERT INTO subscriptions
            (guild_id, channel_id, kind, target, label, ping_role_id, state, created_by)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(new.guild_id.get() as i64)
    .bind(new.channel_id.get() as i64)
    .bind(new.kind.as_str())
    .bind(new.target)
    .bind(new.label)
    .bind(new.ping_role_id.map(|r| r.get() as i64))
    .bind(new.state)
    .bind(new.created_by.get() as i64)
    .execute(db)
    .await?;
    Ok((result.rows_affected() > 0).then(|| result.last_insert_rowid()))
}

pub async fn count_for_guild(db: &SqlitePool, guild: GuildId) -> anyhow::Result<i64> {
    Ok(
        sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions WHERE guild_id = ?")
            .bind(guild.get() as i64)
            .fetch_one(db)
            .await?,
    )
}

pub async fn list_for_guild(db: &SqlitePool, guild: GuildId) -> anyhow::Result<Vec<Subscription>> {
    let rows = sqlx::query_as(select!("WHERE guild_id = ? ORDER BY id"))
        .bind(guild.get() as i64)
        .fetch_all(db)
        .await?;
    convert(rows)
}

pub async fn list_by_kind(db: &SqlitePool, kind: Kind) -> anyhow::Result<Vec<Subscription>> {
    let rows = sqlx::query_as(select!("WHERE kind = ? ORDER BY id"))
        .bind(kind.as_str())
        .fetch_all(db)
        .await?;
    convert(rows)
}

pub async fn find_webhook(db: &SqlitePool, token: &str) -> anyhow::Result<Option<Subscription>> {
    let row: Option<Row> = sqlx::query_as(select!("WHERE kind = 'webhook' AND target = ?"))
        .bind(token)
        .fetch_optional(db)
        .await?;
    row.map(Subscription::try_from).transpose()
}

/// Verwijdert een abonnement, maar alleen binnen de opgegeven server.
pub async fn delete(db: &SqlitePool, guild: GuildId, id: i64) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM subscriptions WHERE id = ? AND guild_id = ?")
        .bind(id)
        .bind(guild.get() as i64)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_guild(db: &SqlitePool, guild: GuildId) -> anyhow::Result<u64> {
    let result = sqlx::query("DELETE FROM subscriptions WHERE guild_id = ?")
        .bind(guild.get() as i64)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}

pub async fn delete_channel(db: &SqlitePool, channel: ChannelId) -> anyhow::Result<u64> {
    let result = sqlx::query("DELETE FROM subscriptions WHERE channel_id = ?")
        .bind(channel.get() as i64)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}

pub async fn set_state(db: &SqlitePool, id: i64, state: Option<&str>) -> anyhow::Result<()> {
    sqlx::query("UPDATE subscriptions SET state = ? WHERE id = ?")
        .bind(state)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

// --- Herinneringen ---

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: i64,
    pub user_id: UserId,
    pub channel_id: Option<ChannelId>,
    pub message: String,
    pub due_at: i64,
    pub created_at: i64,
}

type ReminderRow = (i64, i64, Option<i64>, String, i64, i64);

impl From<ReminderRow> for Reminder {
    fn from((id, user_id, channel_id, message, due_at, created_at): ReminderRow) -> Self {
        Reminder {
            id,
            user_id: UserId::new(user_id as u64),
            channel_id: channel_id.map(|c| ChannelId::new(c as u64)),
            message,
            due_at,
            created_at,
        }
    }
}

pub async fn insert_reminder(
    db: &SqlitePool,
    user: UserId,
    channel: Option<ChannelId>,
    message: &str,
    due_at: i64,
    created_at: i64,
) -> anyhow::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO reminders (user_id, channel_id, message, due_at, created_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user.get() as i64)
    .bind(channel.map(|c| c.get() as i64))
    .bind(message)
    .bind(due_at)
    .bind(created_at)
    .execute(db)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn count_reminders(db: &SqlitePool, user: UserId) -> anyhow::Result<i64> {
    Ok(
        sqlx::query_scalar("SELECT COUNT(*) FROM reminders WHERE user_id = ?")
            .bind(user.get() as i64)
            .fetch_one(db)
            .await?,
    )
}

pub async fn list_reminders(db: &SqlitePool, user: UserId) -> anyhow::Result<Vec<Reminder>> {
    let rows: Vec<ReminderRow> = sqlx::query_as(
        "SELECT id, user_id, channel_id, message, due_at, created_at
         FROM reminders WHERE user_id = ? ORDER BY due_at",
    )
    .bind(user.get() as i64)
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

/// Herinneringen die op of voor `now` vallen, oudste eerst.
pub async fn due_reminders(db: &SqlitePool, now: i64) -> anyhow::Result<Vec<Reminder>> {
    let rows: Vec<ReminderRow> = sqlx::query_as(
        "SELECT id, user_id, channel_id, message, due_at, created_at
         FROM reminders WHERE due_at <= ? ORDER BY due_at LIMIT 50",
    )
    .bind(now)
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

/// Verwijdert een herinnering, maar alleen als ze van deze gebruiker is.
pub async fn delete_reminder(db: &SqlitePool, user: UserId, id: i64) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM reminders WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user.get() as i64)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_reminder_by_id(db: &SqlitePool, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM reminders WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
