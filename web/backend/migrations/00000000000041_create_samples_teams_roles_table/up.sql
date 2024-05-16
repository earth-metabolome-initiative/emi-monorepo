-- Create the samples_teams_roles table.
CREATE TABLE IF NOT EXISTS samples_teams_roles (
    table_id uuid NOT NULL REFERENCES samples(id),
    team_id INTEGER NOT NULL REFERENCES teams(id),
    role_id INTEGER NOT NULL REFERENCES roles(id),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (table_id, team_id, role_id)
);
