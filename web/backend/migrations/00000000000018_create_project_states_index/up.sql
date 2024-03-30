-- Create the index to search approximately the composite columns of
-- project states, including name and description.

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX project_states_name_description_trgm_idx ON project_states USING gin (name gin_trgm_ops, description gin_trgm_ops);