-- Your SQL goes here
CREATE TABLE spectra_collection (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    sample_id UUID REFERENCES samples(id) ON DELETE CASCADE NOT NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a spectra collection is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a spectra collection is deleted.
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
    ON spectra_collection FOR EACH ROW EXECUTE FUNCTION delete_editables();
