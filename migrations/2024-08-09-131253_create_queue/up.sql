-- SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
--
-- SPDX-License-Identifier: AGPL-3.0-only

CREATE TABLE queue(
    id INTEGER PRIMARY KEY NOT NULL,
    job TEXT NOT NULL,
    args TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    locked_at TIMESTAMP
);
