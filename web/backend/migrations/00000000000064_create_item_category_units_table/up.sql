-- SQL defining the item_category_units table.
-- An item category can have multiple units and an unit can belong to multiple item categories.
-- This table is used to store the relationship between item categories and units.
CREATE TABLE IF NOT EXISTS item_category_units (
    id SERIAL PRIMARY KEY,
    item_category_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
    unit_id INTEGER NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    UNIQUE (item_category_id, unit_id)
);