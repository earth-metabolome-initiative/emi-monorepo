-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR,
    middle_name VARCHAR,
    last_name VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- We insert the root user, who is the first user of the system.
-- We will assign to root the creation of the records insert during
-- the migration process.
INSERT INTO
    users (first_name, last_name)
VALUES
    ('root', 'user');