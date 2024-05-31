-- This is a no-op SQL statement
ALTER SEQUENCE observation_subjects_id_seq OWNED BY NONE;
ALTER TABLE observation_subjects ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE observation_subjects_id_seq;