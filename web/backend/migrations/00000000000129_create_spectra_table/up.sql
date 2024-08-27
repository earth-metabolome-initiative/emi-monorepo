-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra (
    feature_id INTEGER NOT NULL,
    spectra_collection_id INTEGER NOT NULL,
    precursor_mz REAL NOT NULL,
    -- We keep the retention time as optional for now to handl, for ex., in silico spectral datababases
    retention_time REAL,
    charge INTEGER NOT NULL,
    -- The following field could be improved to polarity and an enum (positive, negative). We have to chek for GlueSQL compliance here.
    positive BOOLEAN NOT NULL,
    ms_level INTEGER NOT NULL,
    mz REAL[] NOT NULL,
    intensity REAL[] NOT NULL,
    FOREIGN KEY (spectra_collection_id) REFERENCES spectra_collections(id) ON DELETE CASCADE,
    PRIMARY KEY (spectra_collection_id, feature_id, ms_level),
    CHECK (precursor_mz >= 0),
    CHECK (retention_time >= 0),
    CHECK (ms_level >= 1),
    CHECK array_length(mz, 1) = array_length(intensity, 1)
);
