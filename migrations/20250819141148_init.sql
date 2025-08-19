-- Add migration script here

CREATE TABLE IF NOT EXISTS channels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    content_path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS programs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    desc TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    channel_id INTEGER NOT NULL,
    enable BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
);
