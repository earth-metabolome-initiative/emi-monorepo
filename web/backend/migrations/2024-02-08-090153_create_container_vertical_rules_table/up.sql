-- SQL defining the container_vertical_rules table.
-- The container rules define whether an item type can contain another item type.
-- For instance a rack can contain tubes, but a tube cannot contain a rack.
-- We define such rules in an allow-list fashion, meaning that if a rule is not defined,
-- then the item type cannot contain another item type. The rules are defined by an admin
-- user, and are used to enforce the containment rules when creating or updating items.
-- Some containers may only contain items that are within a certain temperature, humidity,
-- or pressure range. These constraints are also defined in the container rules.
CREATE TABLE container_vertical_rules (
  id SERIAL PRIMARY KEY,
  container_item_type_id INTEGER REFERENCES item_types(id),
  contained_item_type_id INTEGER REFERENCES item_types(id),
  temperature INTERVAL DEFAULT NULL,
  humidity INTERVAL DEFAULT NULL,
  pressure INTERVAL DEFAULT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id)
);