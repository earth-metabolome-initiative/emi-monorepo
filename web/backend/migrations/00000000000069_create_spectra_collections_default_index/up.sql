-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE spectra_collections_id_seq;
ALTER TABLE spectra_collections ALTER COLUMN id SET DEFAULT nextval('spectra_collections_id_seq');
ALTER TABLE spectra_collections ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE spectra_collections_id_seq OWNED BY spectra_collections.id;
