-- UP MIGRATION
CREATE TABLE samples (
    id UUID PRIMARY KEY,
    inserted_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE CASCADE,
    sampled_by INTEGER NOT NULL REFERENCES users(id) ON
    DELETE CASCADE,
    procedure_id UUID NOT NULL REFERENCES sampling_procedures(id) ON
    DELETE CASCADE,
    state INTEGER NOT NULL REFERENCES sample_states(id) ON
    DELETE CASCADE
);