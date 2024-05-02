-- Drop sequence ownership and default constraint
ALTER SEQUENCE teams_id_seq OWNED BY NONE;
ALTER TABLE teams ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE teams_id_seq;