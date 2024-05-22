-- This is a no-op SQL statement
ALTER SEQUENCE sample_containers_id_seq OWNED BY NONE;
ALTER TABLE sample_containers ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE sample_containers_id_seq;