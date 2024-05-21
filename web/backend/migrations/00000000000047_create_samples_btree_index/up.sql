-- Creates a BTREE index on the barcode_id UUID column in the samples table.
CREATE INDEX IF NOT EXISTS samples_barcode_id_btree_idx ON samples USING BTREE (barcode_id);