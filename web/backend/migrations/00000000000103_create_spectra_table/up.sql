-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra (
    id INTEGER PRIMARY KEY,
    notes TEXT,
    spectra_collection_id INTEGER REFERENCES spectra_collections(id) ON DELETE CASCADE NOT NULL
);
