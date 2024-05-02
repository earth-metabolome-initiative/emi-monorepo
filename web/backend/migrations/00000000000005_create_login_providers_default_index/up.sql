-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE login_providers_id_seq;
ALTER TABLE login_providers ALTER COLUMN id SET DEFAULT nextval('login_providers_id_seq');
ALTER TABLE login_providers ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE login_providers_id_seq OWNED BY login_providers.id;
