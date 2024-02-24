-- SQL defining the location states table.
-- A location state is a state that a location may be in.
-- A location state is used to describe the state of a location, such as destroyed, at risk, or safe.
CREATE TABLE location_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
    font_awesome_icon VARCHAR(255)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a location state is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a location state is deleted.
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
    ON location_states FOR EACH ROW EXECUTE FUNCTION delete_editables();
