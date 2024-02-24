-- Your SQL goes here
CREATE TABLE describables (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE,
        name TEXT NOT NULL,
        description TEXT
);

-- While we do not expect for it to happen, we add a trigger to delete the corresponding
-- record in the editables table when a describable is deleted. This is to ensure that
-- the database is consistent and that no orphaned records are left behind. We cannot use
-- a cascade delete constraint, as the editables table is referenced by several other tables.
-- Furthermore, not all editables are describables, so we cannot add a cascade delete constraint
-- to the editables table. Instead, we add a trigger to delete the corresponding record in the
-- editables table when a describable is deleted.
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
    ON describables FOR EACH ROW EXECUTE FUNCTION delete_editables();