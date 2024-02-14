-- Your SQL goes here
CREATE TABLE project_user_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);