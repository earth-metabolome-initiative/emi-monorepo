-- This is a no-op SQL statement
ALTER SEQUENCE nameplate_categories_id_seq OWNED BY NONE;
ALTER TABLE nameplate_categories ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE nameplate_categories_id_seq;