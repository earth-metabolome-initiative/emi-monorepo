-- Your SQL goes here
CREATE TABLE taxa (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE REFERENCES describables(id) ON
    DELETE
        CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a taxon is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a taxon is deleted.
CREATE
OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$ BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD .id;

RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON taxa FOR EACH ROW EXECUTE FUNCTION delete_editables();