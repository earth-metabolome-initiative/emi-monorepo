-- SQL defining the item_category_units table.
-- An item category can have multiple units and an unit can belong to multiple item categories.
-- This table is used to store the relationship between item categories and units.
CREATE TABLE item_category_units (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_category_id UUID NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    UNIQUE (item_category_id, unit_id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item category unit is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item category unit is deleted.
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
    ON item_category_units FOR EACH ROW EXECUTE FUNCTION delete_editables();
