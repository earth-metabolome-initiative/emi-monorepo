-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE bio_ott_taxon_items_id_seq;
ALTER TABLE bio_ott_taxon_items ALTER COLUMN id SET DEFAULT nextval('bio_ott_taxon_items_id_seq');
ALTER SEQUENCE bio_ott_taxon_items_id_seq OWNED BY bio_ott_taxon_items.id;