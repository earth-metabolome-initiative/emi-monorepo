-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mass_spectrometry_conversion_methods (
    id INTEGER PRIMARY KEY,
    output_format TEXT NOT NULL,
    -- The following double_precision field is used to specify whether the output format should be encoded in double precision (32 or 64 bits)
    double_precision BOOLEAN NOT NULL,
    zlib_compression BOOLEAN NOT NULL
);