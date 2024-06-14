-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE colors_id_seq;
ALTER TABLE colors ALTER COLUMN id SET DEFAULT nextval('colors_id_seq');
ALTER SEQUENCE colors_id_seq OWNED BY colors.id;