CREATE TABLE IF NOT EXISTS organism_models (
  id INTEGER PRIMARY KEY REFERENCES sample_source_models(id)
);
CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES sample_sources(id),
  organism_model_id INTEGER NOT NULL REFERENCES organism_models(id),
  FOREIGN KEY (id, organism_model_id) REFERENCES assets(id, model_id)
);