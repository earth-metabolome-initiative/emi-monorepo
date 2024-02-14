-- SQL defining the editables table, containing all editable content.
CREATE TABLE editables (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_by INTEGER NOT NULL REFERENCES users(id),
  -- Whether the editable can still be edited or is archived.
  archived BOOLEAN NOT NULL DEFAULT FALSE
);