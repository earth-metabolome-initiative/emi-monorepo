-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX countries_trgm_idx ON countries USING gin (name gin_trgm_ops);