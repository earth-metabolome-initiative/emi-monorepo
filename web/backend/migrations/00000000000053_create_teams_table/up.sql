CREATE TABLE IF NOT EXISTS teams (
    -- change to integer and the change it to serial in a new directoy
    id INTEGER PRIMARY KEY,
    -- a name of the team
    name TEXT NOT NULL UNIQUE,
    -- a description of the team
    description TEXT NOT NULL,
    parent_team_id INTEGER REFERENCES teams(id) ON
    DELETE
        CASCADE,
        -- The user who created the team
        created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
    -- The date the team was created
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
    -- The date the team was last updated
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);