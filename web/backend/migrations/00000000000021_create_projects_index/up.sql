-- Create index to run approximate search queries on projects names and descriptions.
-- The search will be case insensitive and will use the trigram index.

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX projects_name_description_trgm_idx ON projects USING gin (name gin_trgm_ops, description gin_trgm_ops);