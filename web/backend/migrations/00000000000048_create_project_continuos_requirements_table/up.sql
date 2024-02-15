-- Your SQL goes here
CREATE TABLE project_continuous_requirements (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  item_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
  quantity FLOAT NOT NULL,
  unit_id INTEGER REFERENCES units(id) ON DELETE CASCADE,
  UNIQUE (project_id, item_id),
  FOREIGN KEY (item_id, unit_id) REFERENCES item_units(item_id, unit_id) ON DELETE CASCADE,
  FOREIGN KEY (unit_id) REFERENCES continuous_units(id) ON DELETE CASCADE
);