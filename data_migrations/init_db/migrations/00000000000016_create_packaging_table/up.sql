CREATE TABLE IF NOT EXISTS packaging_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);