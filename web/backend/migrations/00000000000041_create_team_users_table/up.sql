-- SQL to create the team_users table.
-- A team user is a user who is a member of a team. When that
-- team or user is deleted, the team_users row should be deleted as well.
-- The role column is used to store the role of the user in the team.
-- The key includes the user_id, team_id, and role columns, as an user can be in a team with different roles.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to a team, the team_users table
-- also contains a column to specify which administrator added the user to the team.
CREATE TABLE team_users (
    id int NOT NULL PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
    user_id int NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    team_id int NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    role_id int NOT NULL REFERENCES team_user_roles (id) ON DELETE CASCADE
);