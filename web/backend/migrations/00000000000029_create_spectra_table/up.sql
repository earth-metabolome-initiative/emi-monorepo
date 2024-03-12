-- Your SQL goes here
CREATE TABLE spectra (
    id SERIAL PRIMARY KEY,
    spectra_collection_id UUID REFERENCES spectra_collection(id) ON DELETE CASCADE NOT NULL
);

-- We also need to add a bi-directional cascade delete constraint to the spectra_collection
-- table, so that when a spectrum is deleted, the corresponding spectra collection is also deleted.
-- Since the spectra_collection table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the spectra_collection table. Instead, we add a trigger to delete the corresponding
-- record in the spectra_collection table when a spectrum is deleted.
CREATE OR REPLACE FUNCTION delete_spectra_collection() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        spectra_collection
    WHERE
        id = OLD.spectra_collection_id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_spectra_collection AFTER
DELETE
    ON spectra FOR EACH ROW EXECUTE FUNCTION delete_spectra_collection();
