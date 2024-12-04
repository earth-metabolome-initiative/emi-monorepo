-- SQL query creating a mockup of the users table.
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    teamsname VARCHAR(255) NOT NULL CHECK (teamsname <> ''),
    email VARCHAR(255) NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
