-- This down migration drops what was created in the up migration.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

ALTER SEQUENCE manufactured_item_categories_id_seq OWNED BY NONE;
ALTER TABLE manufactured_item_categories ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE manufactured_item_categories_id_seq;
