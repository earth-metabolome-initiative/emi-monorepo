-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX bio_ott_ranks_name_trgm_idx ON bio_ott_ranks USING gin (name gin_trgm_ops);