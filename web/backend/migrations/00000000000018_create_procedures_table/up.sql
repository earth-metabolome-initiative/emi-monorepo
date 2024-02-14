-- SQL defining the procedures table.
CREATE TABLE procedures (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT
);