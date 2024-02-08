-- SQL to create the team_users table.
-- A team user is a user who is a member of a team. When that
-- team or user is deleted, the team_users row should be deleted as well.
-- The role column is used to store the role of the user in the team.
-- The key includes the user_id, team_id, and role columns, as an user can be in a team with different roles.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to a team, the team_users table
-- also contains a column to specify which administrator added the user to the team.
CREATE TABLE team_users (
    user_id int NOT NULL,
    team_id int NOT NULL,
    role int NOT NULL,
    added_by int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, team_id, role),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users (id) ON DELETE CASCADE
);