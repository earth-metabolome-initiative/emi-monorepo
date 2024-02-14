-- Your SQL goes here
CREATE TABLE task_discrete_requirements (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  item_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
  quantity FLOAT NOT NULL,
  unit_id INTEGER REFERENCES units(id) ON DELETE CASCADE,
  UNIQUE (task_id, item_id),
  FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id) ON DELETE CASCADE,
  FOREIGN KEY (unit_id) REFERENCES discrete_units(id) ON DELETE CASCADE
);