CREATE TABLE IF NOT EXISTS organism_observations (
    id UUID PRIMARY KEY,
    wild BOOLEAN NOT NULL DEFAULT TRUE,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    organism_id UUID NOT NULL REFERENCES organisms(id),
    subject_id SMALLINT NOT NULL REFERENCES observation_subjects(id),
    picture BYTEA NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (must_be_smaller_than_utc(created_at, updated_at))
);