-- Your SQL goes here
CREATE TABLE spectra_collection (
    id SERIAL PRIMARY KEY,
    sample_id UUID REFERENCES samples(id) ON DELETE CASCADE NOT NULL,
    created_by INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL
);