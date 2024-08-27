-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mass_spectrometry_data_treatment_methods (
    id INTEGER PRIMARY KEY,
    mass_spectrometry_software_id INTEGER NOT NULL,
    other_parameters TEXT,
    FOREIGN KEY (mass_spectrometry_software_id) REFERENCES mass_spectrometry_softwares(id)
);