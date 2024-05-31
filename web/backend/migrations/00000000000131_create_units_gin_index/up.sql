-- Create index to run approximate search queries on the units table.
-- The search will be case insensitive and will use the trigram index.

CREATE FUNCTION concat_units_name_description_symbol(
name text,
description text,
symbol text
) RETURNS text AS $$
BEGIN
RETURN name || ' ' || description || ' ' || symbol;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX units_name_description_symbol_trgm_idx ON units USING gin (
concat_units_name_description_symbol(
name,
description,
symbol
) gin_trgm_ops
);
