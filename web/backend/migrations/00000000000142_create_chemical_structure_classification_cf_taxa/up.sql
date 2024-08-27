CREATE TABLE IF NOT EXISTS chemical_structure_classification_cf_taxa (
    id SERIAL PRIMARY KEY,
    cf_kingdom_id VARCHAR(50) NOT NULL,  -- References CHEMONTID for the kingdom level
    cf_superclass_id VARCHAR(50) NOT NULL,  -- References CHEMONTID for the superclass level
    cf_class_id VARCHAR(50) NOT NULL,  -- References CHEMONTID for the class level
    cf_subclass_id VARCHAR(50),  -- References CHEMONTID for the subclass level (nullable)
    cf_level_5_id VARCHAR(50),  -- References CHEMONTID for level 5 (nullable)
    cf_level_6_id VARCHAR(50),  -- References CHEMONTID for level 6 (nullable)
    cf_level_7_id VARCHAR(50),  -- References CHEMONTID for level 7 (nullable)
    cf_level_8_id VARCHAR(50),  -- References CHEMONTID for level 8 (nullable)
    molecular_framework VARCHAR(255),  -- Describes the molecular framework
    direct_parent_id VARCHAR(50) NOT NULL,  -- References CHEMONTID for the direct parent level
    alternative_parents TEXT,  -- Alternative parents (can be JSON or TEXT)
    substituents TEXT,  -- Substituents (can be JSON or TEXT)
    description TEXT,  -- Description of the chemical structure
    ancestors TEXT,  -- Ancestors (can be JSON or TEXT)
    external_descriptors TEXT,  -- External descriptors like CHEBI terms
    url TEXT,  -- URL linking to the ClassyFire taxonomy node
    FOREIGN KEY (cf_kingdom_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_superclass_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_class_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_subclass_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_level_5_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_level_6_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_level_7_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (cf_level_8_id) REFERENCES chemont_terms(chemont_id),
    FOREIGN KEY (direct_parent_id) REFERENCES chemont_terms(chemont_id)
);
