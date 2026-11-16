-- Eén rij per "volg": een bron (feed, Twitch-kanaal, webhook) die naar een Discord-kanaal uitzendt.
CREATE TABLE subscriptions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id      INTEGER NOT NULL,
    channel_id    INTEGER NOT NULL,
    kind          TEXT    NOT NULL CHECK (kind IN ('rss', 'twitch', 'webhook')),
    -- rss: feed-URL · twitch: login (kleine letters) · webhook: geheime token
    target        TEXT    NOT NULL,
    -- Optionele naam die in meldingen en /volglijst getoond wordt
    label         TEXT,
    ping_role_id  INTEGER,
    -- Integratie-specifieke toestand als JSON (bv. al geziene feed-items, huidige stream-ID)
    state         TEXT,
    created_by    INTEGER NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE (channel_id, kind, target)
);

CREATE INDEX subscriptions_guild ON subscriptions (guild_id);
CREATE INDEX subscriptions_kind ON subscriptions (kind);
CREATE UNIQUE INDEX subscriptions_webhook_token ON subscriptions (target) WHERE kind = 'webhook';
