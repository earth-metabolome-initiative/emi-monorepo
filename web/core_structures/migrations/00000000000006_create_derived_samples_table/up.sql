-- This table defines the relationship between parent and child samples.
-- A parent sample can have multiple child samples, and a child sample can have multiple parent samples.

CREATE TABLE IF NOT EXISTS derived_samples (
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    parent_sample_id UUID NOT NULL,
    child_sample_id UUID NOT NULL,
    quantity FLOAT NOT NULL,
    unit_id SMALLINT NOT NULL,
    PRIMARY KEY (parent_sample_id, child_sample_id),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (child_sample_id) REFERENCES samples(id) ON DELETE CASCADE,
    FOREIGN KEY (unit_id) REFERENCES units(id)
);

