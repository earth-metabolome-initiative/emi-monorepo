CREATE TABLE IF NOT EXISTS soil_models (
  id INTEGER PRIMARY KEY REFERENCES sample_source_models(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS soils (
  id UUID PRIMARY KEY REFERENCES sample_sources(id) ON DELETE CASCADE,
  soil_model_id INTEGER NOT NULL REFERENCES soil_models(id),
  FOREIGN KEY (id, soil_model_id) REFERENCES assets(id, model_id)
);