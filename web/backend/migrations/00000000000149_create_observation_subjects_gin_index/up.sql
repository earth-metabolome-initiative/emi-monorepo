-- Create index to run approximate search queries on the observation_subjects table.
-- The search will be case insensitive and will use the trigram index.
CREATE FUNCTION concat_observation_subjects_name_description(name text, description text) RETURNS text AS $$ BEGIN
    RETURN name || ' ' || description;

END;

$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX observation_subjects_name_description_trgm_idx ON observation_subjects USING gin (
    concat_observation_subjects_name_description(name, description) gin_trgm_ops
);