-- SQL defining the item_category_relationships table.
-- The set of relationships defines a DAG (Directed Acyclic Graph) of item categories.
-- This is a many-to-many relationship between item categories. The relationship is
-- defined by the parent_id and child_id columns. The parent_id column is a foreign key
-- to the id column of the item_categories table. The child_id column is a foreign key
-- to the id column of the item_categories table. The parent_id and child_id columns
-- together form a unique constraint on the table. The created_by and updated_by columns
-- are foreign keys to the id column of the users table. The created_at and updated_at
-- columns are timestamps that are automatically set to the current time when a new
-- row is created or updated.
CREATE TABLE item_category_relationships (
  parent_id INTEGER NOT NULL REFERENCES item_categories(id),
  child_id INTEGER NOT NULL REFERENCES item_categories(id),
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (parent_id, child_id)
);