-- Your SQL goes here
-- A migration to create the sampled_individuals table.
-- An individual is a physically identifiable member of a given species.
CREATE TABLE IF NOT EXISTS sampled_individuals (
  id UUID PRIMARY KEY,
  notes TEXT,
  barcode TEXT,
  -- TODO: add a foreign key to a table with the phisical tags.
  project_id INTEGER NOT NULL REFERENCES projects(id),
  created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Path to the image of the individual.
    picture BYTEA NOT NULL
    -- Geographic coordinates of the individual.
    -- geolocation COORDINATES NOT NULL : TODO!
);