-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra (
    id INTEGER PRIMARY KEY,
    spectra_collection_id INTEGER NOT NULL,
    FOREIGN KEY (spectra_collection_id) REFERENCES spectra_collections(id) ON DELETE CASCADE
);
