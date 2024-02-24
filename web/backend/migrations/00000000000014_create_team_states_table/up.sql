-- SQL defining the team states table.
-- A team state is a state that a team may be in.
-- A team state is used to describe the state of a team, such as active, inactive, or archived.
CREATE TABLE team_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    font_awesome_icon VARCHAR(255)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a team state is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a team state is deleted.
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
    ON team_states FOR EACH ROW EXECUTE FUNCTION delete_editables();
