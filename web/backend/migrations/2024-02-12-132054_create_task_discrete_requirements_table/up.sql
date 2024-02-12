-- Your SQL goes here
CREATE TABLE task_discrete_requirements (
  task_id INTEGER NOT NULL REFERENCES tasks(id),
  item_id INTEGER NOT NULL REFERENCES item_categories(id),
  quantity FLOAT NOT NULL,
  unit_id INTEGER REFERENCES units(id),
  created_by INTEGER REFERENCES users(id),
  updated_by INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (task_id, item_id),
  FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id)
  FOREIGN KEY (unit_id) REFERENCES discrete_units(id)
);