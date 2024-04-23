-- UP MIGRATION
CREATE TABLE samples (
    id UUID PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE CASCADE,
    state INTEGER NOT NULL REFERENCES sample_states(id) ON
    DELETE CASCADE,
    derived_from UUID REFERENCES samples(id) ON
    DELETE
    SET
        NULL
);