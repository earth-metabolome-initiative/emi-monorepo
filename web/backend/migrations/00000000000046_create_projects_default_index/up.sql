-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE projects_id_seq;
ALTER TABLE projects ALTER COLUMN id SET DEFAULT nextval('projects_id_seq');
ALTER TABLE projects ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE projects_id_seq OWNED BY projects.id;
