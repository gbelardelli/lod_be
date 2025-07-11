CREATE TABLE game_session (
    id INTEGER PRIMARY KEY,
    description VARCHAR NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    date_start VARCHAR NOT NULL,
    last_date VARCHAR,
    session_data TEXT NOT NULL
);
