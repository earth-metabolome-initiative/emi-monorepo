-- Your SQL goes here
CREATE TABLE edits (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
  edited_by INTEGER NOT NULL REFERENCES users(id),
  edited_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);