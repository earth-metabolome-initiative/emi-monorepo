-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mass_spectrometry_analysis_converted (
    id INTEGER PRIMARY KEY,
    mass_spectrometry_analysis_raw_id INTEGER NOT NULL,
    -- The following field is called converted_sha has it might refer to mzML but also mzXML or other converted formats. We keep this generic for now
    converted_sha TEXT NOT NULL,
    masss_spectrometry_conversion_method_id INTEGER NOT NULL,
    FOREIGN KEY (mass_spectrometry_analysis_raw_id) REFERENCES mass_spectrometry_analysis_raw_id(id),
    FOREIGN KEY (mass_spectrometry_conversion_method_id) REFERENCES mass_spectrometry_conversion_methods(id)
);