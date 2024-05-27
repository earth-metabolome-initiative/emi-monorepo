-- Create index to run approximate search queries on the organizations table.
-- The search will be case insensitive and will use the trigram index.

CREATE INDEX organizations_name_trgm_idx ON organizations USING gin (
    name gin_trgm_ops
);
