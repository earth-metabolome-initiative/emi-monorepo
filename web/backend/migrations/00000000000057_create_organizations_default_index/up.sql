-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE organizations_id_seq;
ALTER TABLE organizations ALTER COLUMN id SET DEFAULT nextval('organizations_id_seq');
ALTER TABLE organizations ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE organizations_id_seq OWNED BY organizations.id;
