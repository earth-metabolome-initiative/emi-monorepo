-- This migration create a custom data type called jpeg, which validates
-- that the provided BYTEA is a valid JPEG image.

CREATE OR REPLACE FUNCTION js_jpeg(bytea_data BYTEA) RETURNS BOOLEAN AS $$
BEGIN
    -- Check for JPEG (basic check for start of JPEG file)
    RETURN substring(bytea_data for 3) = '\xffd8ff';
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

DROP DOMAIN IF EXISTS jpeg_in CASCADE;
CREATE DOMAIN jpeg_in AS BYTEA
    CONSTRAINT jpeg_in_check CHECK (js_jpeg(VALUE));

DROP TYPE IF EXISTS jpeg CASCADE;
CREATE TYPE jpeg AS (
    value jpeg_in
);