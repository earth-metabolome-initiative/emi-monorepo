-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id INTEGER PRIMARY KEY,
    sample_id UUID REFERENCES samples(id) ON DELETE CASCADE NOT NULL,
    created_by INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL
);