CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY REFERENCES physical_assets(id)
);