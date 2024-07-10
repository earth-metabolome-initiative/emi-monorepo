-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE ranks_id_seq;
ALTER TABLE ranks ALTER COLUMN id SET DEFAULT nextval('ranks_id_seq');
ALTER SEQUENCE ranks_id_seq OWNED BY ranks.id;