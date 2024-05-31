-- Drop index on the sample_containers table.
-- The index was used to run approximate search queries on the table.

DROP INDEX sample_containers_barcode_trgm_idx;
