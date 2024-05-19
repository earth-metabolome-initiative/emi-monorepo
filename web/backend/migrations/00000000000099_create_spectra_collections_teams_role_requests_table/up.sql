-- Create the spectra_collections_teams_role_requests table.
CREATE TABLE IF NOT EXISTS spectra_collections_teams_role_requests (
    table_id integer NOT NULL REFERENCES spectra_collections(id) ON DELETE CASCADE,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (table_id, team_id)
);
