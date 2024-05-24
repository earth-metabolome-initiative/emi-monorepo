-- This is a no-op SQL statement
ALTER SEQUENCE materials_id_seq OWNED BY NONE;
ALTER TABLE materials ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE materials_id_seq;