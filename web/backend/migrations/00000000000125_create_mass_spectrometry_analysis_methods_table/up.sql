-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mass_spectrometry_analysis_methods (
    id INTEGER PRIMARY KEY,
    instrument_id INTEGER NOT NULL,
    -- This is to capture for now the fact that the analysis was runned under positive or negative ionisation mode
    positive BOOLEAN NOT NULL,
    -- This is to capture for now the fact that the analysis was runned in MS1 or MS2 mode
    fragmentation BOOLEAN NOT NULL,
    --- Others parameters are for now captured in the form a free text field
    other_parameters TEXT,
    FOREIGN KEY (instrument_id) REFERENCES instruments(id)
);