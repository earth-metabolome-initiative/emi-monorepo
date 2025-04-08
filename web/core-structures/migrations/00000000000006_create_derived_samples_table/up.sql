-- This table defines the relationship between parent and child samples.
-- A parent sample can have multiple child samples, and a child sample can have multiple parent samples.

CREATE TABLE IF NOT EXISTS derived_samples (
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_sample_id UUID NOT NULL,
    child_sample_id UUID NOT NULL,
    quantity REAL NOT NULL CHECK (must_be_strictly_positive_f32(quantity)),
    unit_id SMALLINT NOT NULL,
    PRIMARY KEY (parent_sample_id, child_sample_id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    FOREIGN KEY (parent_sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (child_sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES units(id)
);
