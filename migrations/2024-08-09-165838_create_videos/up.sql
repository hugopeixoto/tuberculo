-- SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
--
-- SPDX-License-Identifier: AGPL-3.0-only

-- Your SQL goes here
CREATE TABLE videos(
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    duration INTEGER NOT NULL,
    aspect_ratio REAL NOT NULL,
    fulltitle TEXT,
    categories TEXT,
    thumbnail_extension TEXT NOT NULL,
    full_metadata TEXT NOT NULL,

    fetched_at TIMESTAMP NOT NULL
)
