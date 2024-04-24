-- In this SQL up migration, we change the id column of the domains table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE organism_domains_id_seq;
ALTER TABLE organism_domains ALTER COLUMN id SET DEFAULT nextval('organism_domains_id_seq');
ALTER SEQUENCE organism_domains_id_seq OWNED BY organism_domains.id;