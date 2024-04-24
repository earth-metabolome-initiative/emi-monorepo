-- Drop sequence ownership and default constraint
ALTER SEQUENCE classes_id_seq OWNED BY NONE;
ALTER TABLE classes ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE classes_id_seq;