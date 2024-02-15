-- SQL defining the item_continuous_quantities table.
-- Each item, being a physical object, has a weight. This table defines the weights of items.
-- The weight of an item may change over time, and be measured using different scales and by
-- different people.
CREATE TABLE item_continuous_quantities (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    item_id INTEGER REFERENCES items(id) ON DELETE CASCADE,
    weight DECIMAL(10, 2) NOT NULL,
    unit_id INTEGER REFERENCES units(id) ON DELETE CASCADE,
    sensor_id INTEGER REFERENCES items(id) ON DELETE SET NULL,
    measured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    measured_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (item_id, unit_id),
    FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);