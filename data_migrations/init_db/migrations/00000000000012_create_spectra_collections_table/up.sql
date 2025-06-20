-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id INTEGER PRIMARY KEY,
    notes TEXT,
    trackable_id UUID NOT NULL REFERENCES trackables(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_smaller_than_utc(created_at, updated_at))
);