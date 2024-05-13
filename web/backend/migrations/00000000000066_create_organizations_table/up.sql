CREATE TABLE IF NOT EXISTS organizations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    country_id INTEGER REFERENCES countries(id) NOT NULL,
    domain TEXT NOT NULL UNIQUE,
    -- name and country_id must be unique together
    CONSTRAINT organizations_name_country_id_unique UNIQUE (name, country_id)
);