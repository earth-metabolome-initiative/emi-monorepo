-- SQL defining the procedure_discrete_requirements table.
-- A procedure may require one or more items to be performed. This table defines the
-- requirements of a procedure in terms of the number of items of a given category.
CREATE TABLE IF NOT EXISTS procedure_discrete_requirements (
  id SERIAL PRIMARY KEY,
  created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  procedure_id INTEGER NOT NULL REFERENCES procedures(id) ON DELETE CASCADE,
  item_category_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
  quantity INTEGER NOT NULL,
  unit_id INTEGER REFERENCES units(id) ON DELETE CASCADE,
  UNIQUE (procedure_id, item_category_id),
  FOREIGN KEY (item_category_id, unit_id) REFERENCES item_category_units(item_category_id, unit_id) ON DELETE CASCADE,
  FOREIGN KEY (unit_id) REFERENCES discrete_units(id) ON DELETE CASCADE
);
