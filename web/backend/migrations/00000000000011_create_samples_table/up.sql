-- UP MIGRATION
CREATE TABLE samples (
    id UUID PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE,
        derived_from UUID REFERENCES samples(id) ON
    DELETE
    SET
        NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a sample is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a sample is deleted.
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
    ON samples FOR EACH ROW EXECUTE FUNCTION delete_editables();