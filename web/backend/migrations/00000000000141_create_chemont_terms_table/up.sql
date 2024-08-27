CREATE TABLE IF NOT EXISTS chemont_terms (
    chemont_id VARCHAR(50) PRIMARY KEY,  -- CHEMONTID
    name VARCHAR(255) NOT NULL,  -- Name of the classification (e.g., "Benzenoids")
    description TEXT,  -- Description of the classification
    url TEXT  -- URL linking to the ClassyFire taxonomy node
);
