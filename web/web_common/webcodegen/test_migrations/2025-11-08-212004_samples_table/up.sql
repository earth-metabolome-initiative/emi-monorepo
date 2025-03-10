CREATE TABLE samples (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL, 
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    weight REAL NOT NULL CHECK ((must_be_strictly_positive_f32(weight) OR must_be_strictly_greater_than_f32(weight, 0.0)) AND must_be_strictly_smaller_than_f32(weight, 10.0)),
    precision_weight FLOAT NOT NULL CHECK ((must_be_strictly_positive_f64(precision_weight) OR must_be_strictly_greater_than_f64(precision_weight, 0.0)) AND must_be_strictly_smaller_than_f64(precision_weight, 10.0)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL,
    updated_by INTEGER NOT NULL,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);
