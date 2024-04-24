-- Drop sequence ownership and default constraint
ALTER SEQUENCE kingdoms_id_seq OWNED BY NONE;
ALTER TABLE kingdoms ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE kingdoms_id_seq;