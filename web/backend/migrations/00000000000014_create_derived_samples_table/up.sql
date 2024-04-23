-- This table defines the relationship between parent and child samples.
-- A parent sample can have multiple child samples, and a child sample can have multiple parent samples.

CREATE TABLE derived_samples (
    id INTEGER PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_sample_id UUID NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    child_sample_id UUID NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    UNIQUE (parent_sample_id, child_sample_id)
);

