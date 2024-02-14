-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT
);