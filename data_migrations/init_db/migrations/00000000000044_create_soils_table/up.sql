CREATE TABLE IF NOT EXISTS soil_models (
  id INTEGER PRIMARY KEY REFERENCES sample_source_models(id)
);
CREATE TABLE IF NOT EXISTS soils (
  id UUID PRIMARY KEY REFERENCES sample_sources(id),
  model INTEGER NOT NULL REFERENCES soil_models(id),
  FOREIGN KEY (id, model) REFERENCES assets(id, model)
);