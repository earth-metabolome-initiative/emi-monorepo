-- SQL defining the organizations locations table.
-- An organization may be present in one or more locations.
-- An organization may change locations over time.
CREATE TABLE organization_locations (
  editable_id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
  location_id INTEGER REFERENCES locations(id) ON DELETE CASCADE,
  previous_location_id INTEGER REFERENCES organization_locations(editable_id) ON DELETE SET NULL
);
