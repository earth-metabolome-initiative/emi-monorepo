-- Create index to run approximate search queries on the sample_containers table.
-- The search will be case insensitive and will use the trigram index.
CREATE INDEX sample_containers_barcode_trgm_idx ON sample_containers USING gin (
    barcode gin_trgm_ops
);
