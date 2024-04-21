CREATE TABLE item_categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE CASCADE
);