-- SQL creating an index to speed up similarity searches on the name column
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX taxa_name_trgm_idx ON taxa USING gin (name gin_trgm_ops);

