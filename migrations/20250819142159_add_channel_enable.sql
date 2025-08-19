-- Add migration script here

ALTER TABLE channels ADD COLUMN enable BOOLEAN NOT NULL DEFAULT TRUE;
