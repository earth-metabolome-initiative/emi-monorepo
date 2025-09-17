CREATE TABLE IF NOT EXISTS bead_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id),
    diameter_millimeters REAL NOT NULL CHECK (
        must_be_strictly_positive_f32(diameter_millimeters)
    )
);