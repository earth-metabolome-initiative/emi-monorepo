-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id INTEGER PRIMARY KEY,
    mass_spectrometry_conversion_id INTEGER NOT NULL,
    FOREIGN KEY (mass_spectrometry_conversion_id) REFERENCES mass_spectrometry_conversion(id) ON DELETE CASCADE
);