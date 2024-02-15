-- SQL defining the item_category_units table.
-- An item category can have multiple units and an unit can belong to multiple item categories.
-- This table is used to store the relationship between item categories and units.
CREATE TABLE item_category_units (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_category_id BIGINT NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
    unit_id BIGINT NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    UNIQUE (item_category_id, unit_id)
);