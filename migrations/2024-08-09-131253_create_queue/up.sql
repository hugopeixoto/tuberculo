CREATE TABLE queue(
    id INTEGER PRIMARY KEY NOT NULL,
    job TEXT NOT NULL,
    args TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    locked_at TIMESTAMP
);
