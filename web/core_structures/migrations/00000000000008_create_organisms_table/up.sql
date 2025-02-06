-- Your SQL goes here
-- A migration to create the organisms table.
-- An organism is a physically identifiable member of a given species.
CREATE TABLE IF NOT EXISTS organisms (
  id UUID PRIMARY KEY,
  host_organism_id UUID,
  -- This is the optional where this organisms was found in
  sample_id UUID,
  notes TEXT,
  wild BOOLEAN NOT NULL DEFAULT TRUE,
  nameplate_id INTEGER NOT NULL UNIQUE,
  project_id INTEGER NOT NULL,
  created_by INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_by INTEGER NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY (host_organism_id) REFERENCES organisms(id),
  FOREIGN KEY (sample_id) REFERENCES samples(id),
  FOREIGN KEY (nameplate_id) REFERENCES nameplates(id),
  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (created_by) REFERENCES users(id),
  FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT host_organism CHECK (id  != host_organism_id)
);