-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE INDEX taxa_name_trgm_idx ON taxa USING gin (name gin_trgm_ops);
