-- Create the index to search approximately the composite columns of
-- bio_domains, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX bio_domain_items_name_description_trgm_idx ON bio_domain_items USING gin (name gin_trgm_ops, description gin_trgm_ops);