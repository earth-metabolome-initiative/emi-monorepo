-- SQL defining the team_user_roles table.
-- A user may have different roles in different teams.
-- The role column is used to store possible roles of a user in a team.
CREATE TABLE team_user_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);