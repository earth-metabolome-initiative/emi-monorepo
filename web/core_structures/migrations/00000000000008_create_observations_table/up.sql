CREATE TABLE IF NOT EXISTS observations (
    id UUID PRIMARY KEY,
    parent_observation_id UUID,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    project_id INTEGER NOT NULL,
    organism_id UUID,
    sample_id UUID UNIQUE,
    subject_id SMALLINT NOT NULL,
    picture BYTEA NOT NULL,
    notes TEXT,
    FOREIGN KEY (parent_observation_id) REFERENCES observations(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (organism_id) REFERENCES organisms(id),
    FOREIGN KEY (sample_id) REFERENCES samples(id),
    FOREIGN KEY (subject_id) REFERENCES observation_subjects(id),
    CONSTRAINT observation_parent CHECK (id != parent_observation_id)
);