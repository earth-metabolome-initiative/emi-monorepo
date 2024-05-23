-- Create index to run approximate search queries on projects names and descriptions.
-- The search will be case insensitive and will use the trigram index.

CREATE FUNCTION f_concat_projects_name_description(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  CASE
    WHEN description IS NULL THEN
      RETURN name;
    ELSE
      RETURN name || ' ' || description;
  END CASE;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX projects_name_description_trgm_idx ON projects USING gin (
  f_concat_projects_name_description(name, description) gin_trgm_ops
);
