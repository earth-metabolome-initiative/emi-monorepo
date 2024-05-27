-- Create index to run approximate search queries on the sampled_individuals table.
-- The search will be case insensitive and will use the trigram index.
CREATE FUNCTION concat_sampled_individuals_notes_barcode(notes text, barcode text) RETURNS text AS $$ BEGIN
    CASE
        WHEN notes IS NULL
        AND barcode IS NULL THEN RETURN '';

WHEN notes IS NULL THEN RETURN barcode;

WHEN barcode IS NULL THEN RETURN notes;

ELSE RETURN notes || ' ' || barcode;

END CASE;

END;

$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX sampled_individuals_notes_barcode_trgm_idx ON sampled_individuals USING gin (
    concat_sampled_individuals_notes_barcode(notes, barcode) gin_trgm_ops
);