-- Create the index to search approximately the composite columns of
-- sampling_procedures, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX sampling_procedures_name_description_trgm_idx ON sampling_procedures USING gin (name gin_trgm_ops, description gin_trgm_ops);