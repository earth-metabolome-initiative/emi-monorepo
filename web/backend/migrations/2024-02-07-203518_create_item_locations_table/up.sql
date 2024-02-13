-- SQL defining the item_locations table.
-- One extremely important aspect of the Earth Metabolome Initiative is to be able to
-- keep track of the samples, racks and generally speaking all of the notable physical
-- objects that are part of the initiative. This table is the cornerstone of the
-- physical tracking system, and it is the table that will be used to keep track of
-- the location of all of the items that are part of the initiative.
-- It contains the following columns:
-- - id: the unique identifier of the item location.
-- - item_id: the unique identifier of the item that is located at this location.
-- - location_id: the unique identifier of the location where the item is located.
-- - previous_location_id: the unique identifier of the previous location where the item was located.
-- - state: the state of the item at this location, such as "available", "under repair", "in use", "in transit", "moved", "lost", "stolen", "missing", "unknown".
-- - created_at: the date and time when the item location was created.
-- - updated_at: the date and time when the item location was last updated.
-- - created_by: the unique identifier of the user who created the item location.
-- - updated_by: the unique identifier of the user who last updated the item location.
CREATE TABLE item_locations (
  id SERIAL PRIMARY KEY,
  item_id INTEGER REFERENCES items(id),
  location_id INTEGER REFERENCES locations(id),
  previous_location_id INTEGER REFERENCES item_locations(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id)
);