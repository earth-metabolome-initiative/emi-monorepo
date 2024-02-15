-- SQL defining the locations table.
-- A location is a physical place where items can be stored.
-- This table defines the locations where items can be stored.
-- It contains the following columns:
-- - id: the unique identifier of the location.
-- - name: the name of the location.
-- - description: a description of the location.
-- - latitude: the latitude of the location.
-- - longitude: the longitude of the location.
-- - altitude: the altitude of the location.
-- - address: the address of the location, at the best precision available.
-- - parent_location_id: the unique identifier of the parent location, if any.
-- - state: the state of the location, such as "active", "under construction", "decommissioned", "abandoned", "destroyed", "lost", "stolen", "missing", "unknown", "other".
-- - created_at: the date and time when the location was created.
-- - updated_at: the date and time when the location was last updated.
-- - created_by: the unique identifier of the user who created the location.
-- - updated_by: the unique identifier of the user who last updated the location.
CREATE TABLE locations (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE,
  latitude DECIMAL(9,6),
  longitude DECIMAL(9,6),
  altitude DECIMAL(9,3),
  address TEXT,
  geolocalization_device_id INTEGER REFERENCES items(id) ON DELETE SET NULL,
  altitude_device_id INTEGER REFERENCES items(id) ON DELETE SET NULL,
  parent_location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL,
  state_id INTEGER NOT NULL REFERENCES location_states(id)
);
