-- Nieuwe soort 'games' (gratis games van Epic/Steam). SQLite kan een CHECK niet aanpassen,
-- dus de tabel wordt herbouwd met dezelfde kolommen.
CREATE TABLE subscriptions_new (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id      INTEGER NOT NULL,
    channel_id    INTEGER NOT NULL,
    kind          TEXT    NOT NULL CHECK (kind IN ('rss', 'twitch', 'webhook', 'games')),
    -- rss: feed-URL · twitch: login (kleine letters) · webhook: geheime token
    -- games: platform (epic, steam of alle)
    target        TEXT    NOT NULL,
    label         TEXT,
    ping_role_id  INTEGER,
    state         TEXT,
    created_by    INTEGER NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE (channel_id, kind, target)
);

INSERT INTO subscriptions_new
    (id, guild_id, channel_id, kind, target, label, ping_role_id, state, created_by, created_at)
SELECT id, guild_id, channel_id, kind, target, label, ping_role_id, state, created_by, created_at
FROM subscriptions;

DROP TABLE subscriptions;
ALTER TABLE subscriptions_new RENAME TO subscriptions;

CREATE INDEX subscriptions_guild ON subscriptions (guild_id);
CREATE INDEX subscriptions_kind ON subscriptions (kind);
CREATE UNIQUE INDEX subscriptions_webhook_token ON subscriptions (target) WHERE kind = 'webhook';
