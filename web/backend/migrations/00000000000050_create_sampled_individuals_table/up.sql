-- Your SQL goes here
-- A migration to create the sampled_individuals table.
-- An individual is a physically identifiable member of a given species.

CREATE TABLE sampled_individuals (
  id BIGINT PRIMARY KEY REFERENCES items(id) ON DELETE CASCADE
);
