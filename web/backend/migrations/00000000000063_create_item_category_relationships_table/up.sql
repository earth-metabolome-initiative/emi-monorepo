CREATE TABLE IF NOT EXISTS item_category_relationships (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
    child_id INTEGER NOT NULL REFERENCES item_categories(id) ON DELETE CASCADE,
    added_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (parent_id, child_id)
);