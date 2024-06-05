-- Your SQL goes here
-- A migration to create the organisms table.
-- An organism is a physically identifiable member of a given species.
CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY,
  host_organism_id UUID REFERENCES organisms(id),
  -- This is the optional where this organisms was found in
  sample_id UUID REFERENCES samples(id),
  notes TEXT,
  nameplate_id INTEGER NOT NULL UNIQUE REFERENCES nameplates(id),
  -- TODO: add a foreign key to a table with the physical tags.
  project_id INTEGER NOT NULL REFERENCES projects(id),
  created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Path to the image of the organism.
    picture BYTEA NOT NULL
    -- Geographic coordinates of the organism.
    -- geolocation COORDINATES NOT NULL : TODO!
);