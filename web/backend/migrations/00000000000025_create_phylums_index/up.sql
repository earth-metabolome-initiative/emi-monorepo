-- Create the index to search approximately the composite columns of
-- domains, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX phylums_name_trgm_idx ON phylums USING gin (name gin_trgm_ops);