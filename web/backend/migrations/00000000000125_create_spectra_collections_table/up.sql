-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id INTEGER PRIMARY KEY,
    notes TEXT,
    sample_id UUID NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE CASCADE
);