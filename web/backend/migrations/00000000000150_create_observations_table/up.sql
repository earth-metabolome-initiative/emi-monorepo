CREATE TABLE IF NOT EXISTS observations (
    id UUID PRIMARY KEY,
    parent_observation_id UUID REFERENCES observations(id),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    organism_id UUID REFERENCES organisms(id),
    sample_id UUID REFERENCES samples(id),
    subject_id INTEGER NOT NULL REFERENCES observation_subjects(id),
    notes TEXT,
    picture BYTEA NOT NULL
);