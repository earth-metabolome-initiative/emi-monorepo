-- Your SQL goes here
CREATE TABLE edits (
  id SERIAL PRIMARY KEY,
  editable_id INTEGER NOT NULL REFERENCES editables(id),
  edited_by INTEGER NOT NULL REFERENCES users(id),
  edited_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  reason TEXT NOT NULL
);