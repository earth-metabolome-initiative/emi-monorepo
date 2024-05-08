-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_requirements (
    id INTEGER PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE
        CASCADE,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        project_id INTEGER NOT NULL REFERENCES projects(id) ON
    DELETE
        CASCADE,
        item_category_id INTEGER NOT NULL REFERENCES item_categories(id) ON
    DELETE
        CASCADE,
        quantity INTEGER NOT NULL,
        unit_id INTEGER REFERENCES units(id) ON
    DELETE
        CASCADE,
        UNIQUE (project_id, item_category_id),
        FOREIGN KEY (item_category_id, unit_id) REFERENCES item_category_units(item_category_id, unit_id) ON
    DELETE
        CASCADE
);