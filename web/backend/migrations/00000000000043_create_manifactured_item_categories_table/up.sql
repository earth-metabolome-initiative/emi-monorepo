-- SQL defining the manufactured_item_categories table.
-- A manufactured type is an item type that can be manufactured, sold, and bought. As such it
-- has a cost, its currency, and a cost per day for renting and the current of the renting
-- cost.
CREATE TABLE manufactured_item_categories (
  id BIGINT PRIMARY KEY REFERENCES item_categories(id) ON DELETE CASCADE,
  cost DECIMAL(10, 2) NOT NULL,
  cost_per_day DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL,
  manifacturer_id BIGINT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE
);
