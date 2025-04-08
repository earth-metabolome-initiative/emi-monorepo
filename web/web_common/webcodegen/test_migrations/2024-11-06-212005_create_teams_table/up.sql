-- SQL query creating a mockup of the users table.
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    teamsname VARCHAR(255) NOT NULL CHECK (must_not_be_empty(teamsname)),
    email VARCHAR(255) NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS team_members (
    member_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (member_id, team_id)
);