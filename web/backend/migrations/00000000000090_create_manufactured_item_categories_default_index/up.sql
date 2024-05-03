-- This up migration replaces the INTEGER PRIMARY KEY with a INTEGER PRIMARY KEY.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE manufactured_item_categories_id_seq;
ALTER TABLE manufactured_item_categories ALTER COLUMN id SET DEFAULT nextval('manufactured_item_categories_id_seq');
ALTER TABLE manufactured_item_categories ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE manufactured_item_categories_id_seq OWNED BY manufactured_item_categories.id;
