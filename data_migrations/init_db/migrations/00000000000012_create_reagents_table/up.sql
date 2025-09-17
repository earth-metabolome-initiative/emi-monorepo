CREATE TABLE IF NOT EXISTS reagent_models (
  id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
  purity REAL NOT NULL CHECK (
    must_be_strictly_positive_f32(purity)
    AND must_be_smaller_than_f32(purity, 100.0)
  ),
  cas_code CAS NOT NULL UNIQUE,
  molecular_formula MolecularFormula NOT NULL
);