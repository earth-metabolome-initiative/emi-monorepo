-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE units_id_seq;
ALTER TABLE units ALTER COLUMN id SET DEFAULT nextval('units_id_seq');
ALTER TABLE units ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE units_id_seq OWNED BY units.id;
