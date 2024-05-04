CREATE TABLE IF NOT EXISTS item_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE CASCADE
);