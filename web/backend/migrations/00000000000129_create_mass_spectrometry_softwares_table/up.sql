-- This is a no-op SQL statement
CREATE TABLE IF NOT EXISTS mass_spectrometry_softwares(
    id INTEGER PRIMARY KEY,
    mass_spectrometry_software_category_id INTEGER NOT NULL,
    FOREIGN KEY (instrument_category_id) REFERENCES instrument_categories(id)
);