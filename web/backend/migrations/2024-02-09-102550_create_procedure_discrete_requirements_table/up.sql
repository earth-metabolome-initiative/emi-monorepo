-- SQL defining the procedure_discrete_requirements table.
-- A procedure may require one or more items to be performed. This table defines the
-- requirements of a procedure in terms of the number of items of a given category.
CREATE TABLE procedure_discrete_requirements (
  procedure_id INTEGER NOT NULL REFERENCES procedures(id),
  item_category_id INTEGER NOT NULL REFERENCES item_categories(id),
  quantity INTEGER NOT NULL,
  unit_id INTEGER REFERENCES units(id),
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (procedure_id, item_category_id),
  FOREIGN KEY (item_category_id, unit_id) REFERENCES item_category_units(item_id, unit_id)
  FOREIGN KEY (unit_id) REFERENCES discrete_units(id)
);