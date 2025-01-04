CREATE TABLE IF NOT EXISTS protests (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    labels TEXT NOT NULL,
    town TEXT NOT NULL,
    region TEXT NOT NULL,
    country TEXT NOT NULL,
    date TEXT NOT NULL,
    time TEXT NOT NULL,
    place TEXT NOT NULL
);