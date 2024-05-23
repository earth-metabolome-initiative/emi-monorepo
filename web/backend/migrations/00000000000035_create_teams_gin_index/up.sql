-- Create index to run approximate search queries on teams names and descriptions.
-- The search will be case insensitive and will use the trigram index.

CREATE FUNCTION f_concat_teams_name_description(
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

CREATE INDEX teams_name_description_trgm_idx ON teams USING gin (
  f_concat_teams_name_description(name, description) gin_trgm_ops
);
