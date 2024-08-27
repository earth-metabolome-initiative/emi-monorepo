-- Create the projects_teams_role_requests table.
CREATE TABLE IF NOT EXISTS projects_teams_role_requests (
    table_id integer NOT NULL,
    team_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (table_id, team_id),
    FOREIGN KEY (table_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);
