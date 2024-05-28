-- Drop index on the nameplates table.
-- The index was used to run approximate search queries on the table.

DROP INDEX nameplates_barcode_trgm_idx;
