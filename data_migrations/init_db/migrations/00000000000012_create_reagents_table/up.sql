CREATE TABLE IF NOT EXISTS reagent_models (
  id INTEGER PRIMARY KEY REFERENCES asset_models(id) ON DELETE CASCADE,
  purity REAL NOT NULL CHECK (
    purity > 0.0
    AND purity <= 100.0
  ),
  cas_code CAS NOT NULL UNIQUE,
  molecular_formula MolecularFormula NOT NULL
);