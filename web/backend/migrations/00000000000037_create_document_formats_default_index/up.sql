-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE document_formats_id_seq;
ALTER TABLE document_formats ALTER COLUMN id SET DEFAULT nextval('document_formats_id_seq');
ALTER TABLE document_formats ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE document_formats_id_seq OWNED BY document_formats.id;
