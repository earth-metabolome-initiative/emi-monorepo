-- This is a no-op SQL statement
ALTER SEQUENCE sample_container_categories_id_seq OWNED BY NONE;
ALTER TABLE sample_container_categories ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE sample_container_categories_id_seq;