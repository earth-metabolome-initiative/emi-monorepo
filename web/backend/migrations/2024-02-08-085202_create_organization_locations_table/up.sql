-- SQL defining the organizations locations table.
-- An organization may be present in one or more locations.
-- An organization may change locations over time.
CREATE TABLE organization_locations (
  id SERIAL PRIMARY KEY,
  organization_id INTEGER REFERENCES organizations(id),
  location_id INTEGER REFERENCES locations(id),
  previous_location_id INTEGER REFERENCES organization_locations(id),
  state INTEGER NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id)
);
