-- SQL defining the item_units table.
-- An item unit is a unit of measure for an item. For example, an item may be measured in
-- grams, milliliters, or meters. This table defines the units of measure that are used to
-- measure items. Some items may be measured reasonably in different units, and this table
-- allows for the definition of the units of measure that are used to measure items.
CREATE TABLE item_units (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    UNIQUE (item_id, unit_id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item unit is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item unit is deleted.
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
    ON item_units FOR EACH ROW EXECUTE FUNCTION delete_editables();
