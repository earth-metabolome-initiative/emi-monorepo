-- Drop sequence ownership and default constraint
ALTER SEQUENCE colors_id_seq OWNED BY NONE;
ALTER TABLE colors ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE colors_id_seq;