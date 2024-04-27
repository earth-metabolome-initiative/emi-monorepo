-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE OR REPLACE FUNCTION f_concat_sample_states_name_description(
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

CREATE INDEX sample_states_name_description_trgm_idx ON sample_states USING gin (
  f_concat_sample_states_name_description(name, description) gin_trgm_ops
);
