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
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE,
  parent_id INTEGER REFERENCES items(id) ON DELETE SET NULL
);