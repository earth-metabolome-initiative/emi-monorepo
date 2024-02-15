-- SQL defining the procedures table.
CREATE TABLE procedures (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);