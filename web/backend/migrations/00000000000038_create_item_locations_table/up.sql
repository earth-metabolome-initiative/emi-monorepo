-- SQL defining the item_locations table.
-- One extremely important aspect of the Earth Metabolome Initiative is to be able to
-- keep track of the samples, racks and generally speaking all of the notable physical
-- objects that are part of the initiative. This table is the cornerstone of the
-- physical tracking system, and it is the table that will be used to keep track of
-- the location of all of the items that are part of the initiative.
-- It contains the following columns:
-- - id: the unique identifier of the item location.
-- - item_id: the unique identifier of the item that is located at this location.
-- - location_id: the unique identifier of the location where the item is located.
-- - previous_location_id: the unique identifier of the previous location where the item was located.
-- - state: the state of the item at this location, such as "available", "under repair", "in use", "in transit", "moved", "lost", "stolen", "missing", "unknown".
-- - created_at: the date and time when the item location was created.
-- - updated_at: the date and time when the item location was last updated.
-- - created_by: the unique identifier of the user who created the item location.
-- - updated_by: the unique identifier of the user who last updated the item location.
CREATE TABLE item_locations (
  id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  item_id UUID REFERENCES items(id) ON DELETE CASCADE,
  location_id UUID REFERENCES locations(id) ON DELETE CASCADE,
  previous_location_id UUID DEFAULT NULL REFERENCES locations(id) ON DELETE SET NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item location is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item location is deleted.
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
    ON item_locations FOR EACH ROW EXECUTE FUNCTION delete_editables();
