-- This table defines the relationship between parent and child samples.
-- A parent sample can have multiple child samples, and a child sample can have multiple parent samples.

CREATE TABLE IF NOT EXISTS derived_samples (
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_sample_id UUID NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    child_sample_id UUID NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    PRIMARY KEY (parent_sample_id, child_sample_id)
);

