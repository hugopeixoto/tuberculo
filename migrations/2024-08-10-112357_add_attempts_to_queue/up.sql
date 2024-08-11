-- SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
--
-- SPDX-License-Identifier: AGPL-3.0-only

-- Your SQL goes here
ALTER TABLE queue ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;
ALTER TABLE queue ADD COLUMN errors TEXT NOT NULL DEFAULT '';
