-- SQL defining the manufactured_item_types table.
-- A manufactured type is an item type that can be manufactured, sold, and bought. As such it
-- has a cost, its currency, and a cost per day for renting and the current of the renting
-- cost.
CREATE TABLE manufactured_item_types (
  id SERIAL PRIMARY KEY,
  item_type_id INTEGER REFERENCES item_types(id),
  cost DECIMAL(10, 2) NOT NULL,
  cost_per_day DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL,
  manifacturer_id INTEGER REFERENCES organizations(id),
  barcode VARCHAR(255) NOT NULL UNIQUE,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);