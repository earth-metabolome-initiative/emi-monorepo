-- SQL defining the item_continuous_quantities table.
-- Each item, being a physical object, has a weight. This table defines the weights of items.
-- The weight of an item may change over time, and be measured using different scales and by
-- different people.
CREATE TABLE IF NOT EXISTS item_continuous_quantities (
    id UUID PRIMARY KEY,
    item_id UUID REFERENCES items(id) ON DELETE CASCADE NOT NULL,
    amount INTEGER NOT NULL,
    unit_id INTEGER REFERENCES units(id) ON DELETE CASCADE NOT NULL,
    sensor_id UUID REFERENCES items(id) ON DELETE SET NULL,
    measured_at TIMESTAMP NOT NULL DEFAULT now(),
    measured_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (item_id, unit_id),
    FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);
