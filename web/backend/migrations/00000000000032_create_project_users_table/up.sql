-- SQL for creating the project_users table.
-- The project_users table is used to store the users that are associated with a project.
-- The user_id and project_id columns are used to store the user and project, which are used as primary keys.
-- The role column is used to store the role of the user in the project.
-- The created_at column is used to store the creation time of the record.
-- Since only a project administrator can add link a user to a project, the project_users table
-- also contains a column to specify which administrator added the user to the project.

CREATE TABLE project_users (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES project_user_roles (id) ON DELETE CASCADE,
    UNIQUE (user_id, project_id, role_id)
);