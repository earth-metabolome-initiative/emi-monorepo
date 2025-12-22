CREATE TABLE IF NOT EXISTS bead_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE,
    -- Diameter in millimeters
    diameter REAL NOT NULL CHECK (
        diameter > 0.0
    )
);