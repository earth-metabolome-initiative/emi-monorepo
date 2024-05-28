-- This is a no-op SQL statement
CREATE SEQUENCE nameplate_categories_id_seq;
ALTER TABLE nameplate_categories ALTER COLUMN id SET DEFAULT nextval('nameplate_categories_id_seq');
ALTER TABLE nameplate_categories ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE nameplate_categories_id_seq OWNED BY nameplate_categories.id;