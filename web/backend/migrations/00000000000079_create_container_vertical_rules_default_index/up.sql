-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE container_vertical_rules_id_seq;
ALTER TABLE container_vertical_rules ALTER COLUMN id SET DEFAULT nextval('container_vertical_rules_id_seq');
ALTER TABLE container_vertical_rules ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE container_vertical_rules_id_seq OWNED BY container_vertical_rules.id;
