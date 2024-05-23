-- In this SQL up migration, we change the id column of the team states table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE team_states_id_seq;
ALTER TABLE team_states ALTER COLUMN id SET DEFAULT nextval('team_states_id_seq');
ALTER SEQUENCE team_states_id_seq OWNED BY team_states.id;