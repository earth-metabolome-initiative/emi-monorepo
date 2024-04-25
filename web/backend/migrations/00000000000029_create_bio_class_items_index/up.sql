-- Create the index to search approximately the composite columns of
-- domains, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX bio_class_items_name_trgm_idx ON bio_class_items USING gin (name gin_trgm_ops);