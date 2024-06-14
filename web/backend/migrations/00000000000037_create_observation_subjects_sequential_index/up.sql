-- This is a no-op SQL statement
CREATE SEQUENCE observation_subjects_id_seq;
ALTER TABLE observation_subjects ALTER COLUMN id SET DEFAULT nextval('observation_subjects_id_seq');
ALTER TABLE observation_subjects ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE observation_subjects_id_seq OWNED BY observation_subjects.id;