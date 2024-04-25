-- Drop sequence ownership and default constraint
ALTER SEQUENCE taxa_id_seq OWNED BY NONE;
ALTER TABLE taxa ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE taxa_id_seq;