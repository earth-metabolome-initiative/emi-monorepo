-- Your SQL goes here
CREATE TABLE organization_user_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);