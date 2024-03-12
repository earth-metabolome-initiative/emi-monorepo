-- SQL defining the team_user_roles table.
-- A user may have different roles in different teams.
-- The role column is used to store possible roles of a user in a team.
CREATE TABLE team_user_roles (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a team user role is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a team user role is deleted.
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
    ON team_user_roles FOR EACH ROW EXECUTE FUNCTION delete_editables();
