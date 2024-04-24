-- In this SQL up migration, we change the id column of the kingdoms table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE kingdoms_id_seq;
ALTER TABLE kingdoms ALTER COLUMN id SET DEFAULT nextval('kingdoms_id_seq');
ALTER SEQUENCE kingdoms_id_seq OWNED BY kingdoms.id;