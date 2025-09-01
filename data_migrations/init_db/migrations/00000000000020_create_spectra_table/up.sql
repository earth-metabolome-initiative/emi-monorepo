CREATE TABLE IF NOT EXISTS spectra (
    id UUID PRIMARY KEY REFERENCES digital_assets(id) ON DELETE CASCADE,
    spectra_collection_id UUID NOT NULL REFERENCES spectra_collections(id) ON DELETE CASCADE
);
