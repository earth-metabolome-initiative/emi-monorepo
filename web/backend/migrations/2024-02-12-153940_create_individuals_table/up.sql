-- Your SQL goes here
-- A migration to create the individuals table.
-- An individual is a physically identifiable member of a given species.

CREATE TABLE individuals (
  id SERIAL PRIMARY KEY,
  item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE
);
