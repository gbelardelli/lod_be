CREATE TABLE game_session (
    id INTEGER PRIMARY KEY,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 'f',
    date_start VARCHAR NOT NULL,
    last_date VARCHAR,
    session_data TEXT NOT NULL
);
