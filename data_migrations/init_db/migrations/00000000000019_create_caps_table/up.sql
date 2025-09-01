CREATE TABLE IF NOT EXISTS caps_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);