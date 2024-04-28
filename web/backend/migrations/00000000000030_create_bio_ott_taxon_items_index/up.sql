-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX bio_ott_taxon_items_name_trgm_idx ON bio_ott_taxon_items USING gin (name gin_trgm_ops);
-- CREATE INDEX bio_ott_taxon_items_name_trgm_idx ON bio_ott_taxon_items USING gist (name gist_trgm_ops);