-- This is a no-op SQL statement
CREATE SEQUENCE sample_container_categories_id_seq;
ALTER TABLE sample_container_categories ALTER COLUMN id SET DEFAULT nextval('sample_container_categories_id_seq');
ALTER TABLE sample_container_categories ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE sample_container_categories_id_seq OWNED BY sample_container_categories.id;