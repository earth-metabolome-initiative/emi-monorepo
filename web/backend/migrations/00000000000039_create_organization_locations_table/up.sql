-- SQL defining the organizations locations table.
-- An organization may be present in one or more locations.
-- An organization may change locations over time.
CREATE TABLE organization_locations (
  id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
  location_id UUID REFERENCES locations(id) ON DELETE CASCADE,
  previous_location_id UUID REFERENCES organization_locations(id) ON DELETE SET NULL
);


-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a organization location is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a organization location is deleted.
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
    ON organization_locations FOR EACH ROW EXECUTE FUNCTION delete_editables();
