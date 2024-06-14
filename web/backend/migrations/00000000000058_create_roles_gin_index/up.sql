CREATE FUNCTION concat_roles_name(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX roles_name_trgm_idx ON roles USING gin (
  concat_roles_name(name, description) gin_trgm_ops
);