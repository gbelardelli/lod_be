CREATE TABLE character_species (
    id INTEGER PRIMARY KEY,
    name VARCHAR(32),
    description TEXT,
    notes TEXT,
    start_stats TEXT
);

CREATE TABLE character_profession (
    id INTEGER PRIMARY KEY,
    name VARCHAR(32),
    description TEXT,
    start_gear TEXT,
    notes TEXT,
    start_stats TEXT
);

CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    color INTEGER NOT NULL CHECK (color >= 0),
    roles INTEGER NOT NULL CHECK (roles >= 0),
    password VARCHAR NOT NULL
);

CREATE TABLE game_character (
    id INTEGER PRIMARY KEY,
    player_id INT NOT NULL,
    specie_id INT NOT NULL,
    profession_id INT NOT NULL,
    name TEXT NOT NULL,
    level INT DEFAULT 1,
    experience INT DEFAULT 0,
    condition TEXT,
    comment TEXT,
    stats TEXT,      -- CharacterStats
    skills TEXT,     -- CharacterSkills
    effects TEXT,    -- GenericGameEffect

    FOREIGN KEY(player_id) REFERENCES player(id),
    FOREIGN KEY(specie_id) REFERENCES character_species(id),
    FOREIGN KEY(profession_id) REFERENCES character_profession(id)
);



CREATE TABLE game_character_session (
    game_character_id INTEGER NOT NULL,
    game_session_id INTEGER NOT NULL,
    session_stats TEXT,

    PRIMARY KEY(game_character_id, game_session_id)
);

CREATE TABLE character_notes (
    id INTEGER PRIMARY KEY,
    character_id INTEGER,
    note TEXT, -- Il background va qui

    FOREIGN KEY(character_id) REFERENCES game_character(id)
);

CREATE TABLE weapon (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    damage TEXT NOT NULL DEFAULT '',
    encumbrance SMALLINT NOT NULL DEFAULT 0,
    class SMALLINT NOT NULL DEFAULT 0,
    special TEXT,
    cost INTEGER NOT NULL,
    availability SMALLINT NOT NULL,
    reload SMALLINT NOT NULL DEFAULT 0
);