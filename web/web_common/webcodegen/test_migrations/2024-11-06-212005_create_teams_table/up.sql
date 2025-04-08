-- SQL query creating a mockup of the users table.
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL CHECK (must_not_be_empty(name)),
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS team_members (
    member_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (member_id, team_id)
);