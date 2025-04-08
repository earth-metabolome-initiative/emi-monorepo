-- SQL query creating a mockup of the spectra table.
CREATE TABLE spectra (
    id SERIAL PRIMARY KEY,
    spectra_hash TEXT NOT NULL,
    spectra_producer INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL,
    updated_by INTEGER NOT NULL,
    UNIQUE (spectra_hash),
    FOREIGN KEY (spectra_producer) REFERENCES users(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);
