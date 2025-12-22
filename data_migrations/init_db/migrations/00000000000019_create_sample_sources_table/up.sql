CREATE TABLE IF NOT EXISTS sample_source_models (
  id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS sample_sources (
  id UUID PRIMARY KEY REFERENCES physical_assets(id) ON DELETE CASCADE,
  sample_source_model_id INTEGER NOT NULL REFERENCES sample_source_models(id),
  FOREIGN KEY (id, sample_source_model_id) REFERENCES assets(id, model_id)
);