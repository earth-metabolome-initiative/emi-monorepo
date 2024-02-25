-- SQL to create the team_users table.
-- A team user is a user who is a member of a team. When that
-- team or user is deleted, the team_users row should be deleted as well.
-- The role column is used to store the role of the user in the team.
-- The key includes the user_id, team_id, and role columns, as an user can be in a team with different roles.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to a team, the team_users table
-- also contains a column to specify which administrator added the user to the team.
CREATE TABLE team_users (
    id BIGINT PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    team_id BIGINT NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES team_user_roles (id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a team user is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a team user is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON team_users FOR EACH ROW EXECUTE FUNCTION delete_editables();
