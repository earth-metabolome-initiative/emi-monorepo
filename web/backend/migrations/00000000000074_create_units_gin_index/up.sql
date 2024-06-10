-- Create index to run approximate search queries on the units table.
-- The search will be case insensitive and will use the trigram index.

CREATE OR REPLACE FUNCTION concat_units_name_unit(
name text,
unit text
) RETURNS text AS $$
BEGIN
RETURN name || ' ' || unit;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX units_name_unit_trgm_idx ON units USING gin (
concat_units_name_unit(
name,
unit
) gin_trgm_ops
);
