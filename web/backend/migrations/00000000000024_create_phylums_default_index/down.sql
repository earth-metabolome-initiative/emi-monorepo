-- Drop sequence ownership and default constraint
ALTER SEQUENCE phylums_id_seq OWNED BY NONE;
ALTER TABLE phylums ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE phylums_id_seq;