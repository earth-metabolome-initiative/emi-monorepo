-- SQL defining the item_types table.
-- An item type is a type of item that can be tracked and managed, may have a cost,
-- and may be rented. As such it as two different columns one for the cost, its currency,
-- and a cost per day for renting and the current of the renting cost. An item may have
-- an expiration date and may be associated with one or more projects (which is tracked
-- in the item_projects table). An item type may define an item that has been manifactured,
-- such as a tube, a rack, a freezer, a microscope, etc. Additional item types may be more
-- organic such as edible products (bread, cheese, etc.). Some items may be instead collected
-- in nature such as rocks, plants, etc. The item_types table contains a name column to store
-- the name of the item type, a description column to store a description of the item type,
-- the created_at and updated_at columns to store the creation and last update time of the record,
-- plus who created and last updated the record. An item type may also have a manifacturer.
CREATE TABLE item_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  cost DECIMAL(10, 2) DEFAULT NULL,
  currency VARCHAR(3) DEFAULT NULL,
  cost_per_day DECIMAL(10, 2) DEFAULT NULL,
  current_rental_cost DECIMAL(10, 2) DEFAULT NULL,
  expiration_date DATE DEFAULT NULL,
  manufacturer INTEGER REFERENCES organizations(id) DEFAULT NULL,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);