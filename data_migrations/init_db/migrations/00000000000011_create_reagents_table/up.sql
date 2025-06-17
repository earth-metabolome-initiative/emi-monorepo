CREATE TABLE IF NOT EXISTS reagents (
  id UUID PRIMARY KEY REFERENCES trackables(id) ON DELETE CASCADE,
  purity REAL NOT NULL CHECK (
    must_be_strictly_positive_f32(purity)
    AND must_be_smaller_than_f32(purity, 100.0)
  ),
  cas_code CAS NOT NULL UNIQUE,
  molecular_formula MolecularFormula NOT NULL
);