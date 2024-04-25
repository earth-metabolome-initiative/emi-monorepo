-- In this SQL up migration, we change the id column of the bio_class_items table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE bio_class_items_id_seq;
ALTER TABLE bio_class_items ALTER COLUMN id SET DEFAULT nextval('bio_class_items_id_seq');
ALTER SEQUENCE bio_class_items_id_seq OWNED BY bio_class_items.id;