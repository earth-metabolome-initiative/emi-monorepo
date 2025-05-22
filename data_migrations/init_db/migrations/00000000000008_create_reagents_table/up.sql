CREATE TABLE IF NOT EXISTS reagents (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
  description TEXT NOT NULL CHECK (must_be_paragraph(description)),
  purity REAL NOT NULL CHECK (
    must_be_strictly_positive_f32(purity)
    AND must_be_smaller_than_f32(purity, 100.0)
  ),
  cas_code CAS NOT NULL UNIQUE,
  molecular_formula MolecularFormula NOT NULL,
  created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);