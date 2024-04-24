-- Create the index to search approximately the composite columns of
-- domains, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX organism_domains_name_description_trgm_idx ON organism_domains USING gin (name gin_trgm_ops, description gin_trgm_ops);