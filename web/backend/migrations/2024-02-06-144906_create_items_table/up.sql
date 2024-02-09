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
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  created_by UUID NOT NULL,
  updated_by UUID NOT NULL,
  project_id UUID NOT NULL,
  parent_id UUID,
  location_id UUID,
  FOREIGN KEY (created_by) REFERENCES users(id),
  FOREIGN KEY (updated_by) REFERENCES users(id),
  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (parent_id) REFERENCES items(id),
  FOREIGN KEY (item_type_id) REFERENCES item_types(id),
  FOREIGN KEY (location_id) REFERENCES locations(id)
);