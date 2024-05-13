CREATE TABLE IF NOT EXISTS item_locations (
    id UUID PRIMARY KEY,
    item_id UUID REFERENCES items(id) ON
    DELETE
        CASCADE,
        located_by INTEGER REFERENCES users(id) ON
    DELETE
    SET
        NULL,
        located_at TIMESTAMP NOT NULL DEFAULT now(),
        location_id UUID REFERENCES locations(id) ON
    DELETE
        CASCADE
);