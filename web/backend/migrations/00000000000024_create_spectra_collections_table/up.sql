-- Your SQL goes here
CREATE TABLE spectra_collection (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    sample_id INTEGER REFERENCES samples(id) ON DELETE CASCADE NOT NULL
);