CREATE TABLE IF NOT EXISTS sample_models (
  id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id),
  sample_source_model_id INTEGER NOT NULL REFERENCES sample_source_models(id),
	-- We create a unique index to allow for foreign keys checking that there exist a `sample_source_model`
	-- for the current `sample_model`.
  UNIQUE (id, sample_source_model_id)
);

CREATE TABLE IF NOT EXISTS samples (
  id UUID PRIMARY KEY REFERENCES physical_assets(id),
  sample_model_id INTEGER NOT NULL REFERENCES sample_models(id),
  sample_source_id UUID REFERENCES sample_sources(id),
  sample_source_model_id INTEGER NOT NULL REFERENCES sample_source_models(id),
  FOREIGN KEY (id, sample_model_id) REFERENCES assets(id, model_id),
  FOREIGN KEY (sample_model_id, sample_source_model_id) REFERENCES sample_models(id, sample_source_model_id),
  FOREIGN KEY (sample_source_id, sample_source_model_id) REFERENCES assets(id, model_id)
);