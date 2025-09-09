CREATE TABLE IF NOT EXISTS sample_models (
  id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id)
);

CREATE TABLE IF NOT EXISTS samples (
  id UUID PRIMARY KEY REFERENCES physical_assets(id),
  model INTEGER NOT NULL REFERENCES sample_models(id),
  sample_source UUID NOT NULL REFERENCES sample_sources(id),
  FOREIGN KEY (id, model) REFERENCES assets(id, model)
);