CREATE TABLE IF NOT EXISTS observations (
    id UUID PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    individual_id UUID REFERENCES sampled_individuals(id),
    notes TEXT,
    picture BYTEA NOT NULL
);