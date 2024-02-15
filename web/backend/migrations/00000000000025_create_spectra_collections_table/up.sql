-- Your SQL goes here
CREATE TABLE spectra_collection (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    sample_id BIGINT REFERENCES samples(id) ON DELETE CASCADE NOT NULL
);