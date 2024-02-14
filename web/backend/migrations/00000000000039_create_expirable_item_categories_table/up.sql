-- SQL defining the expirable_item_categories table.
-- Item types appearing in this table have an expiration date, meaning that
-- they have an interval of time associated to them after which they are no longer
-- valid. This table is used to enforce the expiration date of items of a certain type.
CREATE TABLE expirable_item_categories (
  editable_id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  item_type_id INTEGER REFERENCES item_categories(id) ON DELETE CASCADE,
  expiration_interval INTERVAL NOT NULL
);