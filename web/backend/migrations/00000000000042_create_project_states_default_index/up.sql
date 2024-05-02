-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE project_states_id_seq;
ALTER TABLE project_states ALTER COLUMN id SET DEFAULT nextval('project_states_id_seq');
ALTER TABLE project_states ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE project_states_id_seq OWNED BY project_states.id;
