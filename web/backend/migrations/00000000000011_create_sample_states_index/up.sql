-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX sample_states_name_description_trgm_idx ON sample_states USING gin (name gin_trgm_ops, description gin_trgm_ops);