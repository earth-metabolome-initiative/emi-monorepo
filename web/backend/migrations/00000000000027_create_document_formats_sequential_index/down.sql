-- This down migration drops what was created in the up migration.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

ALTER SEQUENCE document_formats_id_seq OWNED BY NONE;
ALTER TABLE document_formats ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE document_formats_id_seq;
