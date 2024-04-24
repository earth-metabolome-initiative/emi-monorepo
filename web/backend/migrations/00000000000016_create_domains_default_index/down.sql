-- Drop sequence ownership and default constraint
ALTER SEQUENCE domains_id_seq OWNED BY NONE;
ALTER TABLE domains ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE domains_id_seq;