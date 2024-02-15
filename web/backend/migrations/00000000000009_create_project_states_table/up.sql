-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);