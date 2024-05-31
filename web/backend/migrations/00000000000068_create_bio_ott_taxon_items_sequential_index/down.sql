-- Drop sequence ownership and default constraint
ALTER SEQUENCE bio_ott_taxon_items_id_seq OWNED BY NONE;
ALTER TABLE bio_ott_taxon_items ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE bio_ott_taxon_items_id_seq;