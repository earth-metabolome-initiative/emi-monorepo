-- Create the index to search approximately the composite columns of
-- project states, including name and description.
CREATE INDEX countries_trgm_idx ON countries USING gin (name gin_trgm_ops);