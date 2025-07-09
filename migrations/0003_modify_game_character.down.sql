-- Add down migration script here
ALTER TABLE game_character DROP COLUMN hp;
ALTER TABLE game_character DROP COLUMN sanity;
ALTER TABLE game_character DROP COLUMN luck;
ALTER TABLE game_character DROP COLUMN energy;
