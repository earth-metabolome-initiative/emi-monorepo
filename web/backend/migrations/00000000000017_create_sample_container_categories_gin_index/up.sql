-- This is a no-op SQL statement
CREATE FUNCTION f_concat_sample_container_categories_brand(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX sample_container_categories_trgm_idx ON sample_container_categories USING gin (
  f_concat_sample_container_categories_brand(name, description) gin_trgm_ops
);