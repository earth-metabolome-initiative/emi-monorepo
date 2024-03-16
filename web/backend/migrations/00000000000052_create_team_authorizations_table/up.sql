-- SQL defining the team_authorizations table.
CREATE TABLE team_authorizations (
    team_id UUID NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    editable_id UUID NOT NULL REFERENCES editables (id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, editable_id, role_id)
);
