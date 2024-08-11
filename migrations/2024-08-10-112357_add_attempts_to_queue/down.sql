-- SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
--
-- SPDX-License-Identifier: AGPL-3.0-only

-- This file should undo anything in `up.sql`
ALTER TABLE queue DROP COLUMN errors;
ALTER TABLE queue DROP COLUMN attempts;
