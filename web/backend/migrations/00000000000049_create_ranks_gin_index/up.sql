-- Create the index to search approximately the composite columns of
-- taxonomic ranks, including name and description.
CREATE OR REPLACE FUNCTION concat_ranks_name_description(
  name text,
  description text
) RETURNS text AS $$
BEGIN
    RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX ranks_name_description_trgm_idx ON ranks USING gin (
  concat_ranks_name_description(name, description) gin_trgm_ops
);
