-- Create the index to search approximately the composite columns of
-- taxonomic ranks, including name and description.
CREATE OR REPLACE FUNCTION concat_bio_ott_ranks_name_description(
  name text,
  description text
) RETURNS text AS $$
BEGIN
    RETURN name || ' ' || description;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX bio_ott_ranks_name_description_trgm_idx ON bio_ott_ranks USING gin (
  concat_bio_ott_ranks_name_description(name, description) gin_trgm_ops
);
