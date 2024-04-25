-- Drop sequence ownership and default constraint
ALTER SEQUENCE bio_order_items_id_seq OWNED BY NONE;
ALTER TABLE bio_order_items ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE bio_order_items_id_seq;