-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE roles_id_seq;
ALTER TABLE roles ALTER COLUMN id SET DEFAULT nextval('roles_id_seq');
ALTER TABLE roles ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE roles_id_seq OWNED BY roles.id;
