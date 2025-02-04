CREATE TABLE IF NOT EXISTS nameplates(
    id INTEGER PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL DEFAULT 1,
    geolocation geometry(POINT, 4326) NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES nameplate_categories(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);
