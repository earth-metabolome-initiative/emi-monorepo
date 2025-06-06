-- UP MIGRATION
CREATE TABLE IF NOT EXISTS processables (
    id UUID PRIMARY KEY NOT NULL REFERENCES trackables(id),
    -- The current amount of the processable
    kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms))
);

CREATE TABLE IF NOT EXISTS volumetric_processables (
    id UUID PRIMARY KEY REFERENCES processables(id),
    -- The current volume of the trackable
    liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters))
);