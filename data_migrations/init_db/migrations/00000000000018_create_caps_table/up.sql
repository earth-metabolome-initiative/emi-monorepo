CREATE TABLE IF NOT EXISTS cap_models (
    id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);