-- Create index to run approximate search queries on the nameplates table.
-- The search will be case insensitive and will use the trigram index.
CREATE INDEX nameplates_barcode_trgm_idx ON nameplates USING gin (
    barcode gin_trgm_ops
);
