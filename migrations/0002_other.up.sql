
CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    player_name VARCHAR NOT NULL,
    color INTEGER
);

CREATE TABLE game_character (
    id INTEGER PRIMARY KEY,
    player_id INTEGER NOT NULL,
    specie_id INT NOT NULL,
    profession_id INT NOT NULL,
    name TEXT NOT NULL,
    level INT DEFAULT 1,
    experience INT DEFAULT 0,
    stats TEXT NOT NULL,      -- CharacterStats
    skills TEXT NOT NULL,     -- CharacterSkills
    effects TEXT NOT NULL,    -- GenericGameEffect

    FOREIGN KEY(player_id) REFERENCES player(id),
    FOREIGN KEY(specie_id) REFERENCES character_species(id),
    FOREIGN KEY(profession_id) REFERENCES character_profession(id)
);

CREATE TABLE character_species (
    id INTEGER PRIMARY KEY,
    name INTEGER
);

CREATE TABLE character_profession (
    id INTEGER PRIMARY KEY,
    name INTEGER
);

CREATE TABLE game_character_session (
    game_character_id INTEGER NOT NULL,
    game_session_id INTEGER NOT NULL,
    session_stats TEXT NOT NULL,

    PRIMARY KEY(game_character_id, game_session_id)
);

CREATE TABLE character_notes (
    id INTEGER PRIMARY KEY,
    character_id INTEGER,
    note TEXT, -- Il background va qui

    FOREIGN KEY(character_id) REFERENCES game_character(id)
);
