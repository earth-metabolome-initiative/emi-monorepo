-- SQL defining the item_continuous_quantities table.
-- Each item, being a physical object, has a weight. This table defines the weights of items.
-- The weight of an item may change over time, and be measured using different scales and by
-- different people.
CREATE TABLE item_continuous_quantities (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id UUID REFERENCES items(id) ON DELETE CASCADE NOT NULL,
    weight FLOAT NOT NULL,
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE NOT NULL,
    sensor_id UUID REFERENCES items(id) ON DELETE SET NULL,
    measured_at TIMESTAMP NOT NULL DEFAULT now(),
    measured_by UUID REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (item_id, unit_id),
    FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item continuous quantity is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item continuous quantity is deleted.
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
    ON item_continuous_quantities FOR EACH ROW EXECUTE FUNCTION delete_editables();