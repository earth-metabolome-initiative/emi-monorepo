-- This is a no-op SQL statement
CREATE FUNCTION concat_nameplate_categories_brand(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX nameplate_categories_trgm_idx ON nameplate_categories USING gin (
  concat_nameplate_categories_brand(name, description) gin_trgm_ops
);