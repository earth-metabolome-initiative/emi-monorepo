-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE
);