-- Your SQL goes here
-- A migration to create the sampled_individuals table.
-- An individual is a physically identifiable member of a given species.
CREATE TABLE IF NOT EXISTS sampled_individuals (
  id UUID PRIMARY KEY,
  created_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name TEXT,
    -- Whether the individual has been tagged physically.
    tagged BOOLEAN NOT NULL DEFAULT FALSE
    -- Path to the image of the individual.
    -- image_path TEXT, TODO!
    -- Geographic coordinates of the individual.
    -- geolocation COORDINATES NOT NULL : TODO!
);