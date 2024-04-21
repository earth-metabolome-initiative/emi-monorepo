-- UP MIGRATION
CREATE TABLE samples (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_by INTEGER REFERENCES users(id) ON
    DELETE
        CASCADE,
        derived_from UUID REFERENCES samples(id) ON
    DELETE
    SET
        NULL
);