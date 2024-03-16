-- SQL defining the container_vertical_rules table.
-- The container rules define whether an item type can contain another item type.
-- For instance a rack can contain tubes, but a tube cannot contain a rack.
-- We define such rules in an allow-list fashion, meaning that if a rule is not defined,
-- then the item type cannot contain another item type. The rules are defined by an admin
-- user, and are used to enforce the containment rules when creating or updating items.
-- Some containers may only contain items that are within a certain temperature, humidity,
-- or pressure range. These constraints are also defined in the container rules.
CREATE TABLE container_vertical_rules (
  id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  container_item_type_id UUID REFERENCES item_categories(id) ON DELETE CASCADE,
  contained_item_type_id UUID REFERENCES item_categories(id) ON DELETE CASCADE,
  temperature NUMRANGE DEFAULT NULL,
  humidity NUMRANGE DEFAULT NULL,
  pressure NUMRANGE DEFAULT NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a container vertical rule is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a container vertical rule is deleted.
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
    ON container_vertical_rules FOR EACH ROW EXECUTE FUNCTION delete_editables();