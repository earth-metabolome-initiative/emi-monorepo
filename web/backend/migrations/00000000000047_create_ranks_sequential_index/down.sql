-- Drop sequence ownership and default constraint
ALTER SEQUENCE ranks_id_seq OWNED BY NONE;
ALTER TABLE ranks ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE ranks_id_seq;