-- Drop sequence ownership and default constraint
ALTER SEQUENCE sample_states_id_seq OWNED BY NONE;
ALTER TABLE sample_states ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE sample_states_id_seq;