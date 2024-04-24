-- In this SQL up migration, we change the id column of the phylums table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE phylums_id_seq;
ALTER TABLE phylums ALTER COLUMN id SET DEFAULT nextval('phylums_id_seq');
ALTER SEQUENCE phylums_id_seq OWNED BY phylums.id;