-- SQL defining the item_weights table.
-- Each item, being a physical object, has a weight. This table defines the weights of items.
-- The weight of an item may change over time, and be measured using different scales and by
-- different people.
CREATE TABLE item_weights (
    id SERIAL PRIMARY KEY,
    item_id INTEGER REFERENCES items(id),
    weight DECIMAL(10, 2) NOT NULL,
    weight_unit_id INTEGER REFERENCES weight_units(id),
    sensor_id INTEGER REFERENCES items(id),
    measured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    measured_by INTEGER REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by INTEGER NOT NULL REFERENCES users(id),
    updated_by INTEGER NOT NULL REFERENCES users(id),
    FOREIGN KEY (item_id, weight_unit_id) REFERENCES items(item_id, weight_unit_id),
);