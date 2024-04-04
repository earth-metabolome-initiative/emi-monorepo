-- SQL to create the projects table.
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    state_id UUID NOT NULL REFERENCES project_states(id),
    parent_project_id UUID REFERENCES projects(id) ON
    DELETE
        CASCADE,
        budget FLOAT DEFAULT NULL,
        expenses FLOAT DEFAULT NULL,
        created_by UUID NOT NULL REFERENCES users(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expected_end_date TIMESTAMP DEFAULT NULL,
        end_date TIMESTAMP DEFAULT NULL
);