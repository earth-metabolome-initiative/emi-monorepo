-- SQL defining the procedures table.
CREATE TABLE procedures (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);