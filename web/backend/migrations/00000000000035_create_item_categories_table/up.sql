-- SQL defining the item_categories table.
-- An item type is a type of item that can be tracked and managed, may have a cost,
-- and may be rented. As such it as two different columns one for the cost, its currency,
-- and a cost per day for renting and the current of the renting cost. An item may have
-- an expiration date and may be associated with one or more projects (which is tracked
-- in the item_projects table).
CREATE TABLE item_categories (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT
);