-- In this SQL up migration, we change the id column of the classes table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE classes_id_seq;
ALTER TABLE classes ALTER COLUMN id SET DEFAULT nextval('classes_id_seq');
ALTER SEQUENCE classes_id_seq OWNED BY classes.id;