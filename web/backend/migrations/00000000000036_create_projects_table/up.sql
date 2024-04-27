-- SQL to create the projects table.
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    state_id INTEGER NOT NULL REFERENCES project_states(id),
    parent_project_id INTEGER REFERENCES projects(id) ON
    DELETE
        CASCADE,
        budget BIGINT DEFAULT NULL,
        expenses BIGINT DEFAULT NULL,
        created_by INTEGER NOT NULL REFERENCES users(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expected_end_date TIMESTAMP DEFAULT NULL,
        end_date TIMESTAMP DEFAULT NULL
);