-- Drop index on the sampled_individuals table.
-- The index was used to run approximate search queries on the table.

DROP INDEX sampled_individuals_notes_barcode_trgm_idx;
DROP FUNCTION f_concat_sampled_individuals_notes_barcode(notes text,
barcode text,
);
