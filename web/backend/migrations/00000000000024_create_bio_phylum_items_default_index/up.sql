-- In this SQL up migration, we change the id column of the bio_phylum_items table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE bio_phylum_items_id_seq;
ALTER TABLE bio_phylum_items ALTER COLUMN id SET DEFAULT nextval('bio_phylum_items_id_seq');
ALTER SEQUENCE bio_phylum_items_id_seq OWNED BY bio_phylum_items.id;