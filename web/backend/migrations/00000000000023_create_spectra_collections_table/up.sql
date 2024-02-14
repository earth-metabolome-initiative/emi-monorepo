-- Your SQL goes here
CREATE TABLE spectra_collection (
    id SERIAL PRIMARY KEY,
    sample_id INTEGER REFERENCES samples(id) ON DELETE CASCADE NOT NULL,
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);