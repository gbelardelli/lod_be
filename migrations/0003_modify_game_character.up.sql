-- Add up migration script here
ALTER TABLE game_character ADD COLUMN hp INT NOT NULL;
ALTER TABLE game_character ADD COLUMN sanity INT NOT NULL;
ALTER TABLE game_character ADD COLUMN luck INT DEFAULT 0;
ALTER TABLE game_character ADD COLUMN energy INT;
