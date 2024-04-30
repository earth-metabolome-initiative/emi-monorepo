-- Your SQL goes here
-- A migration to create the sampled_individuals table.
-- An individual is a physically identifiable member of a given species.

CREATE TABLE IF NOT EXISTS sampled_individuals (
  id UUID PRIMARY KEY
);