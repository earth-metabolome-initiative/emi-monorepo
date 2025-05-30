CREATE TABLE IF NOT EXISTS chemical_entities (
  -- TODO! This primary should be a CAS - Chemical Abstracts Service - number
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
  description TEXT NOT NULL CHECK (must_be_paragraph(description)),
  purity REAL NOT NULL CHECK (
    must_be_strictly_positive_f32(purity)
    AND must_be_strictly_smaller_than_f32(purity, 100.0)
  ),
  gram_per_mole REAL NOT NULL CHECK (must_be_strictly_positive_f32(gram_per_mole)),
  created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);