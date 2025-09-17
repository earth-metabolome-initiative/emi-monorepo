CREATE TABLE IF NOT EXISTS sample_source_models (
  id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);
CREATE TABLE IF NOT EXISTS sample_sources (
  id UUID PRIMARY KEY REFERENCES physical_assets(id),
  model INTEGER NOT NULL REFERENCES sample_source_models(id),
  FOREIGN KEY (id, model) REFERENCES assets(id, model)
);