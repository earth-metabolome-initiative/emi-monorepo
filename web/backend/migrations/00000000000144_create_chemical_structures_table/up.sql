CREATE TABLE IF NOT EXISTS chemical_structures (
    chemical_structure_id INTEGER PRIMARY KEY,
    name VARCHAR(255),  -- Optional name for the compound
    smiles TEXT NOT NULL,  -- SMILES string representation
    inchi TEXT NOT NULL,  -- InChI string representation
    inchikey CHAR(27) NOT NULL,  -- InChIKey (27 characters long)
    exact_mass REAL NOT NULL,  -- Exact mass of the compound
    molecular_formula VARCHAR(100),  -- Molecular formula
    molecular_weight REAL,  -- Molecular weight (can be different from exact mass)
    UNIQUE(inchikey),
    CHECK (exact_mass >= 0),
    CHECK (molecular_weight >= 0)
);
