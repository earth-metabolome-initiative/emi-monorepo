CREATE TABLE IF NOT EXISTS container_vertical_rules (
    id INTEGER PRIMARY KEY,
    created_by INTEGER REFERENCES users(id) ON
    DELETE
        CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_by INTEGER REFERENCES users(id) ON
    DELETE
        CASCADE NOT NULL,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        name TEXT NOT NULL UNIQUE,
        container_item_type_id INTEGER REFERENCES item_categories(id) ON
    DELETE
        CASCADE NOT NULL,
        contained_item_type_id INTEGER REFERENCES item_categories(id) ON
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