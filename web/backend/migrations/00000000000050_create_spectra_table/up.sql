-- Your SQL goes here
CREATE TABLE spectra (
    id SERIAL PRIMARY KEY,
    spectra_collection_id INTEGER REFERENCES spectra_collection(id) ON DELETE CASCADE NOT NULL
);
