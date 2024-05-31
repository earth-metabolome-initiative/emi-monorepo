-- This is a no-op SQL statement
CREATE SEQUENCE materials_id_seq;
ALTER TABLE materials ALTER COLUMN id SET DEFAULT nextval('materials_id_seq');
ALTER TABLE materials ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE materials_id_seq OWNED BY materials.id;