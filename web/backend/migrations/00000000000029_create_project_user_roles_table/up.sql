-- Your SQL goes here
CREATE TABLE project_user_roles (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE
);