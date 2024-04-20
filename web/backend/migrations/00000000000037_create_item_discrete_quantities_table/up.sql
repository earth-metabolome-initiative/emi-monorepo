-- SQL defining the item_quantities table.
-- Some items may be counted discretely, as opposed to a weight in grams.
-- This table defines the discrete quantities of items that we have in stock.
-- The quantity of an item may change over time, so multiple quantity entries
-- may be inserted for the same item.
-- While these quantities are discrete, in order to facilitate the semantics of
-- the system, we will still allow for the user to specify the counter unit for 
-- the item. For example, a counter unit may be a box, a tube, or a vial.
CREATE TABLE item_discrete_quantities (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id UUID REFERENCES items(id) ON DELETE CASCADE NOT NULL,
    quantity INTEGER NOT NULL,
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE NOT NULL,
    measured_at TIMESTAMP NOT NULL DEFAULT now(),
    measured_by UUID REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (item_id, unit_id),
    FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES discrete_units(id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item discrete quantity is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item discrete quantity is deleted.
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
    ON item_discrete_quantities FOR EACH ROW EXECUTE FUNCTION delete_editables();
