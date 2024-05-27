-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE countries_id_seq;
ALTER TABLE countries ALTER COLUMN id SET DEFAULT nextval('countries_id_seq');
ALTER TABLE countries ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE countries_id_seq OWNED BY countries.id;
