-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);