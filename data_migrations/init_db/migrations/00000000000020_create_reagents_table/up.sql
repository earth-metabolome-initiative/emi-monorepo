CREATE TABLE IF NOT EXISTS reagents (
  id INTEGER PRIMARY KEY REFERENCES trackable_categories(id) ON DELETE CASCADE,
  purity REAL NOT NULL CHECK (
    must_be_strictly_positive_f32(purity)
    AND must_be_smaller_than_f32(purity, 100.0)
  ),
  cas_code CAS NOT NULL UNIQUE,
  molecular_formulas MolecularFormula NOT NULL,
  created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);