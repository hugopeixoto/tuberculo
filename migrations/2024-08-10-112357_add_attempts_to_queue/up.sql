-- Your SQL goes here
ALTER TABLE queue ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;
ALTER TABLE queue ADD COLUMN errors TEXT NOT NULL DEFAULT '';