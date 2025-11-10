CREATE TABLE IF NOT EXISTS bead_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id),
    diameter_millimeters REAL NOT NULL CHECK (
        diameter_millimeters > 0.0
    )
);