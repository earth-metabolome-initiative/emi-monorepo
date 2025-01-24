-- This is a no-op SQL statement
CREATE SEQUENCE sample_containers_id_seq;
ALTER TABLE sample_containers ALTER COLUMN id SET DEFAULT nextval('sample_containers_id_seq');
ALTER TABLE sample_containers ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE sample_containers_id_seq OWNED BY sample_containers.id;
