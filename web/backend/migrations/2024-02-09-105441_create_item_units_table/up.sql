-- SQL defining the item_units table.
-- An item unit is a unit of measure for an item. For example, an item may be measured in
-- grams, milliliters, or meters. This table defines the units of measure that are used to
-- measure items. Some items may be measured reasonably in different units, and this table
-- allows for the definition of the units of measure that are used to measure items.
CREATE TABLE item_units (
    item_id INTEGER NOT NULL REFERENCES items(id),
    unit_id INTEGER NOT NULL REFERENCES units(id),
    created_by INTEGER REFERENCES users(id),
    updated_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (item_id, unit_id)
);