-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE procedures_id_seq;
ALTER TABLE procedures ALTER COLUMN id SET DEFAULT nextval('procedures_id_seq');
ALTER TABLE procedures ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE procedures_id_seq OWNED BY procedures.id;
