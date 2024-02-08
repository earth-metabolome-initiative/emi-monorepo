-- SQL defining the expirable_item_types table.
-- Item types appearing in this table have an expiration date, meaning that
-- they have an interval of time associated to them after which they are no longer
-- valid. This table is used to enforce the expiration date of items of a certain type.
CREATE TABLE expirable_item_types (
  id SERIAL PRIMARY KEY,
  item_type_id INTEGER REFERENCES item_types(id),
  expiration_interval INTERVAL NOT NULL,
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);