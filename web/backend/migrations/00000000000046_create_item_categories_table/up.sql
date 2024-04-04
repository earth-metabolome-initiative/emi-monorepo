-- SQL defining the item_categories table.
-- An item type is a type of item that can be tracked and managed, may have a cost,
-- and may be rented. As such it as two different columns one for the cost, its currency,
-- and a cost per day for renting and the current of the renting cost. An item may have
-- an expiration date and may be associated with one or more projects (which is tracked
-- in the item_projects table).
CREATE TABLE item_categories (
  id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item category is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item category is deleted.
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
    ON item_categories FOR EACH ROW EXECUTE FUNCTION delete_editables();
