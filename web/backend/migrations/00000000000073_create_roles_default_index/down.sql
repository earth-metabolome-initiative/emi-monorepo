-- This down migration drops what was created in the up migration.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

ALTER SEQUENCE roles_id_seq OWNED BY NONE;
ALTER TABLE roles ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE roles_id_seq;
