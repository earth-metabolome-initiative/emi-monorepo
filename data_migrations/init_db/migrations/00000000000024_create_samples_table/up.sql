CREATE TABLE IF NOT EXISTS sample_models (
  id INTEGER PRIMARY KEY REFERENCES physical_asset_models(id),
  sample_source_model INTEGER NOT NULL REFERENCES sample_source_models(id),
	-- We create a unique index to allow for foreign keys checking that there exist a `sample_source_model`
	-- for the current `sample_model`.
  UNIQUE (id, sample_source_model)
);

CREATE TABLE IF NOT EXISTS samples (
  id UUID PRIMARY KEY REFERENCES physical_assets(id),
  sample_model INTEGER NOT NULL REFERENCES sample_models(id),
  sample_source UUID REFERENCES sample_sources(id),
  sample_source_model INTEGER NOT NULL REFERENCES sample_source_models(id),
  FOREIGN KEY (id, sample_model) REFERENCES assets(id, model),
  FOREIGN KEY (sample_model, sample_source_model) REFERENCES sample_models(id, sample_source_model),
  FOREIGN KEY (sample_source, sample_source_model) REFERENCES assets(id, model)
);