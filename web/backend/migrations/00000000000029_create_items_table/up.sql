-- SQL defining the items table.
-- An item is a physical or digital object that can be tracked and managed.
-- Items can be associated with one or more projects, and can be part of other items.
-- The ownership of an item may change over time, and an item may be associated with
-- one or more users. An item may have a parent item, and may be a container of other
-- items. This table defines the items that are tracked and managed by the system.
-- An item can be movable or immovable.
-- An example of an Item may be a measurement device, a tube potentially containing a sample,
-- or a sample itself.
CREATE TABLE items (
  id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE,
  parent_id UUID REFERENCES items(id) ON DELETE SET NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an item is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an item is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM editables WHERE id = OLD.id;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER DELETE ON items FOR EACH ROW EXECUTE FUNCTION delete_editables();
