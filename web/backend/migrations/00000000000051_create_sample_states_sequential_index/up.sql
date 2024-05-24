-- In this SQL up migration, we change the id column of the taxa table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE sample_states_id_seq;
ALTER TABLE sample_states ALTER COLUMN id SET DEFAULT nextval('sample_states_id_seq');
ALTER SEQUENCE sample_states_id_seq OWNED BY sample_states.id;