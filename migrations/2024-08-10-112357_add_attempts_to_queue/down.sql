-- This file should undo anything in `up.sql`
ALTER TABLE queue DROP COLUMN errors;
ALTER TABLE queue DROP COLUMN attempts;
