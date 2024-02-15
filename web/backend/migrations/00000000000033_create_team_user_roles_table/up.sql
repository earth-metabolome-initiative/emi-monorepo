-- SQL defining the team_user_roles table.
-- A user may have different roles in different teams.
-- The role column is used to store possible roles of a user in a team.
CREATE TABLE team_user_roles (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE
);