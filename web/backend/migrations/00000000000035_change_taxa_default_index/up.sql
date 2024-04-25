-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE taxa_id_seq;
ALTER TABLE taxa ALTER COLUMN id SET DEFAULT nextval('taxa_id_seq');
ALTER SEQUENCE taxa_id_seq OWNED BY taxa.id;