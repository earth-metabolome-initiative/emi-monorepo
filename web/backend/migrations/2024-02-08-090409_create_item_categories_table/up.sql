-- SQL defining the item_categories table.
-- An item type is a type of item that can be tracked and managed, may have a cost,
-- and may be rented. As such it as two different columns one for the cost, its currency,
-- and a cost per day for renting and the current of the renting cost. An item may have
-- an expiration date and may be associated with one or more projects (which is tracked
-- in the item_projects table).
CREATE TABLE item_categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);