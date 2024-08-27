-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mass_spectrometry_analysis_raw (
    id INTEGER PRIMARY KEY,
    sample_id UUID NOT NULL,
    mass_spectrometry_analysis_method_id INTEGER NOT NULL,
    raw_sha TEXT NOT NULL,
    -- The following field is called converted_sha has it might refer to mzML but also mzXML or other converted formats. We keep this generic for now
    instrument_id INTEGER NOT NULL,
    FOREIGN KEY (sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (instrument_id) REFERENCES instruments(id),
    FOREIGN KEY (mass_spectrometry_analysis_method_id) REFERENCES mass_spectrometry_analysis_methods(id)
);