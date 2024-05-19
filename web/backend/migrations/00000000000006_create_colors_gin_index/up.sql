CREATE FUNCTION f_concat_colors_name(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX colors_name_trgm_idx ON colors USING gin (
  f_concat_colors_name(name, description) gin_trgm_ops
);