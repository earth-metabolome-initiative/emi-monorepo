-- SQL defining the container_horizontal_rules table.
-- The container horizontal rules define whether an item type can be placed next to another item type.
-- For instance a acid product cannot be placed next to a base product. Generally speaking, most items
-- can be placed next to each other, but some items cannot be placed next to each other. These rules
-- are defined in the form of a deny-list, meaning that if a rule is not defined, then the item type
-- can be placed next to any other item type. The rules are defined by an admin user, and are used to
-- enforce the placement rules when creating or updating items. Some items may only be placed next to
-- items that are within a certain temperature, humidity, or pressure range. These constraints are also
-- defined in the container rules.
CREATE TABLE container_horizontal_rules (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  item_type_id INTEGER REFERENCES item_categories(id) ON DELETE CASCADE,
  other_item_type_id INTEGER REFERENCES item_categories(id) ON DELETE CASCADE,
  temperature INTERVAL DEFAULT NULL,
  humidity INTERVAL DEFAULT NULL,
  pressure INTERVAL DEFAULT NULL
);