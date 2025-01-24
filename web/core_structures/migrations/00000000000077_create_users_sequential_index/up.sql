-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE users_id_seq;
ALTER TABLE users ALTER COLUMN id SET DEFAULT nextval('users_id_seq');
ALTER TABLE users ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE users_id_seq OWNED BY users.id;
