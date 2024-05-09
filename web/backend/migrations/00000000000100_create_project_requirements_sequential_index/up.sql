-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE project_requirements_id_seq;
ALTER TABLE project_requirements ALTER COLUMN id SET DEFAULT nextval('project_requirements_id_seq');
ALTER TABLE project_requirements ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE project_requirements_id_seq OWNED BY project_requirements.id;
