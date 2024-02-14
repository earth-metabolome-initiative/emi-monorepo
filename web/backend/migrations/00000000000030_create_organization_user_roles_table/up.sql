-- Your SQL goes here
CREATE TABLE organization_user_roles (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);