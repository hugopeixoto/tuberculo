-- Your SQL goes here
CREATE TABLE videos(
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    duration INTEGER NOT NULL,
    aspect_ratio REAL NOT NULL,
    fulltitle TEXT,
    categories TEXT,
    full_metadata TEXT NOT NULL,

    fetched_at TIMESTAMP NOT NULL
)
