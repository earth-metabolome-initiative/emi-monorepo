-- Your SQL goes here
-- A migration to create the sampled_individuals table.
-- An individual is a physically identifiable member of a given species.

CREATE TABLE sampled_individuals (
  id BIGINT PRIMARY KEY REFERENCES items(id) ON DELETE CASCADE
);


-- We also need to add a bi-directional cascade delete constraint to the items
-- table, so that when a sampled individual is deleted, the corresponding item is also deleted.
-- Since the items table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the items table. Instead, we add a trigger to delete the corresponding
-- record in the items table when a sampled individual is deleted.
CREATE OR REPLACE FUNCTION delete_items() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        items
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_items AFTER
DELETE
    ON sampled_individuals FOR EACH ROW EXECUTE FUNCTION delete_items();
