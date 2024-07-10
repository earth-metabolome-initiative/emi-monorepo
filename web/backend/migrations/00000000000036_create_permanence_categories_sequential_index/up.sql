CREATE SEQUENCE permanence_categories_id_seq;
ALTER TABLE permanence_categories ALTER COLUMN id SET DEFAULT nextval('permanence_categories_id_seq');
ALTER TABLE permanence_categories ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE permanence_categories_id_seq OWNED BY permanence_categories.id;