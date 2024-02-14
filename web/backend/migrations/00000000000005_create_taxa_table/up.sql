-- Your SQL goes here
CREATE TABLE taxa (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(255) NOT NULL,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);