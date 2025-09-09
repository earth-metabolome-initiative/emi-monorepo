CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES sample_sources(id)
);