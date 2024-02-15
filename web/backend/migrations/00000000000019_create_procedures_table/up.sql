-- SQL defining the procedures table.
CREATE TABLE procedures (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describable(id) ON DELETE CASCADE
);