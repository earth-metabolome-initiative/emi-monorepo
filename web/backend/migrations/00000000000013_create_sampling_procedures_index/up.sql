-- Create the index to search approximately the composite columns of
-- sampling_procedures, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE FUNCTION f_concat_sampling_procedures_name_description(
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

CREATE INDEX sampling_procedures_name_description_trgm_idx ON sampling_procedures USING gin (
  f_concat_sampling_procedures_name_description(name, description) gin_trgm_ops
);
