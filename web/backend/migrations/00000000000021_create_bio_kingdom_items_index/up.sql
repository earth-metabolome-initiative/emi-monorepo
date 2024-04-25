-- Create the index to search approximately the composite columns of
-- domains, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX bio_kingdom_items_name_description_trgm_idx ON bio_kingdom_items USING gin (name gin_trgm_ops, description gin_trgm_ops);