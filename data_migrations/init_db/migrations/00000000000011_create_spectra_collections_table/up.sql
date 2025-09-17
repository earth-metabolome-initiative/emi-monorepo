-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id UUID PRIMARY KEY REFERENCES digital_assets(id) ON DELETE CASCADE
);