-- Drop sequence ownership and default constraint
ALTER SEQUENCE bio_kingdom_items_id_seq OWNED BY NONE;
ALTER TABLE bio_kingdom_items ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE bio_kingdom_items_id_seq;