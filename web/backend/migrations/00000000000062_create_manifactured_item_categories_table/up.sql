-- SQL defining the manufactured_item_categories table.
-- A manufactured type is an item type that can be manufactured, sold, and bought. As such it
-- has a cost, its currency, and a cost per day for renting and the current of the renting
-- cost.
CREATE TABLE IF NOT EXISTS manufactured_item_categories (
  id SERIAL PRIMARY KEY,
  cost INTEGER NOT NULL,
  cost_per_day INTEGER NOT NULL,
  currency VARCHAR(3) NOT NULL,
  manifacturer_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE
);
