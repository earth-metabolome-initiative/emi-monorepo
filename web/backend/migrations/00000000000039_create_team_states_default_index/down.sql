-- Drop sequence ownership and default constraint
ALTER SEQUENCE team_states_id_seq OWNED BY NONE;
ALTER TABLE team_states ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE team_states_id_seq;