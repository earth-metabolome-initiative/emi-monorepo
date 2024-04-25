-- In this SQL up migration, we change the id column of the domains table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE bio_domain_items_id_seq;
ALTER TABLE bio_domain_items ALTER COLUMN id SET DEFAULT nextval('bio_domain_items_id_seq');
ALTER SEQUENCE bio_domain_items_id_seq OWNED BY bio_domain_items.id;