-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE item_category_relationships_id_seq;
ALTER TABLE item_category_relationships ALTER COLUMN id SET DEFAULT nextval('item_category_relationships_id_seq');
ALTER TABLE item_category_relationships ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE item_category_relationships_id_seq OWNED BY item_category_relationships.id;
