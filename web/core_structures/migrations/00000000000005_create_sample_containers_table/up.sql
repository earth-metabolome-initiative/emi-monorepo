CREATE TABLE IF NOT EXISTS sample_containers(
    id INTEGER PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL DEFAULT 1,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES sample_container_categories(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);

CREATE INDEX sample_containers_barcode_trgm_idx ON sample_containers USING gin (
    barcode gin_trgm_ops
);
