-- SQL defining the organizations locations table.
-- An organization may be present in one or more locations.
-- An organization may change locations over time.
CREATE TABLE organization_locations (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  organization_id BIGINT REFERENCES organizations(id) ON DELETE CASCADE,
  location_id BIGINT REFERENCES locations(id) ON DELETE CASCADE,
  previous_location_id BIGINT REFERENCES organization_locations(id) ON DELETE SET NULL
);
