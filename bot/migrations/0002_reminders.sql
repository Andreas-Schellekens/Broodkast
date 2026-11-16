-- Herinneringen van /herinner. Tijden zijn Unix-seconden (UTC).
CREATE TABLE reminders (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER NOT NULL,
    -- Kanaal om in te posten; NULL = via DM (bv. bij user-install of in een DM)
    channel_id  INTEGER,
    message     TEXT    NOT NULL,
    due_at      INTEGER NOT NULL,
    created_at  INTEGER NOT NULL
);

CREATE INDEX reminders_due ON reminders (due_at);
CREATE INDEX reminders_user ON reminders (user_id);
