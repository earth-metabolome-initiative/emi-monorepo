-- SQL defining the item_category_units table.
-- An item category can have multiple units and an unit can belong to multiple item categories.
-- This table is used to store the relationship between item categories and units.
CREATE TABLE item_category_units (
    item_category_id INTEGER NOT NULL REFERENCES item_categories(id),
    unit_id INTEGER NOT NULL REFERENCES units(id),
    created_by INTEGER REFERENCES users(id),
    updated_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (item_category_id, unit_id)
);