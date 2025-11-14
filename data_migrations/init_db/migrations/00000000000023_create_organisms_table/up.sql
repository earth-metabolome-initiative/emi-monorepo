CREATE TABLE IF NOT EXISTS organism_models (
  id INTEGER PRIMARY KEY REFERENCES sample_source_models(id)
);
CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES sample_sources(id),
  organism_model INTEGER NOT NULL REFERENCES organism_models(id),
  FOREIGN KEY (id, organism_model) REFERENCES sample_sources(id, sample_source_model)
);