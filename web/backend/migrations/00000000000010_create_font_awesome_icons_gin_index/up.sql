CREATE FUNCTION f_concat_font_awesome_icons_name(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX font_awesome_icons_name_trgm_idx ON font_awesome_icons USING gin (
  f_concat_font_awesome_icons_name(name, description) gin_trgm_ops
);