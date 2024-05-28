-- This is a no-op SQL statement
ALTER SEQUENCE permanence_categories_id_seq OWNED BY NONE;
ALTER TABLE permanence_categories ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE permanence_categories_id_seq;