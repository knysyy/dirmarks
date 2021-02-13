CREATE TABLE histories
(
    id   INTEGER PRIMARY KEY,
    path VARCHAR(255)  NOT NULL,
    count INTEGER DEFAULT 0 NOT NULL,
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')) NOT NULL,
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime')) NOT NULL
);
