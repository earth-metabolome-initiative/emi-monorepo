-- In this SQL up migration, we change the id column of the bio_order_items table
-- from INTEGER to SERIAL, which will automatically generate unique
-- values for each row.

CREATE SEQUENCE bio_order_items_id_seq;
ALTER TABLE bio_order_items ALTER COLUMN id SET DEFAULT nextval('bio_order_items_id_seq');
ALTER SEQUENCE bio_order_items_id_seq OWNED BY bio_order_items.id;