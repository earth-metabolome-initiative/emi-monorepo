-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra_collections (
    id INTEGER PRIMARY KEY,
    mass_spectrometry_analysis_converted_id INTEGER NOT NULL,
    mass_spectrometry_data_treament_method_id INTEGER NOT NULL,
    FOREIGN KEY (mass_spectrometry_analysis_converted_id) REFERENCES mass_spectrometry_analysis_converted(id) ON DELETE CASCADE,
    FOREIGN KEY (mass_spectrometry_data_treament_method_id) REFERENCES mass_spectrometry_data_treament_method(id) ON DELETE CASCADE
);