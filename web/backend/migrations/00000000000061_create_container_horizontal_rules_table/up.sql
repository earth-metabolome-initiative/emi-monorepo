CREATE TABLE IF NOT EXISTS container_horizontal_rules (
    id SERIAL PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        name TEXT NOT NULL UNIQUE,
        item_type_id INTEGER REFERENCES item_categories(id) ON
    DELETE
        CASCADE NOT NULL,
        other_item_type_id INTEGER REFERENCES item_categories(id) ON
    DELETE
        CASCADE NOT NULL,
        minimum_temperature INTEGER DEFAULT NULL,
        maximum_temperature INTEGER DEFAULT NULL,
        minimum_humidity INTEGER DEFAULT NULL,
        maximum_humidity INTEGER DEFAULT NULL,
        minimum_pressure INTEGER DEFAULT NULL,
        maximum_pressure INTEGER DEFAULT NULL,
        CHECK (
            minimum_temperature IS NULL
            OR maximum_temperature IS NULL
            OR minimum_temperature <= maximum_temperature
        ),
        CHECK (
            minimum_humidity IS NULL
            OR maximum_humidity IS NULL
            OR minimum_humidity <= maximum_humidity
        ),
        CHECK (
            minimum_pressure IS NULL
            OR maximum_pressure IS NULL
            OR minimum_pressure <= maximum_pressure
        )
);