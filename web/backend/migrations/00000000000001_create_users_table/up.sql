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

-- We import the function that will notify the user when a record is
-- inserted, updated or deleted.
CREATE
OR REPLACE FUNCTION notify_user() RETURNS TRIGGER AS $$
DECLARE
    record JSON;
BEGIN
    IF TG_OP = 'INSERT'
    OR TG_OP = 'UPDATE' THEN record = row_to_json(NEW);

ELSE record = row_to_json(OLD);

END IF;


PERFORM pg_notify(
    'notify_user_' || OLD.id :: text,
    json_build_object(
        'table', TG_TABLE_NAME,
        'operation', TG_OP,
        'record', record
    ) :: text
);

RETURN NEW;

END;

$$ LANGUAGE plpgsql;

-- We create the triggers that will notify the user when a record is
-- inserted, updated or deleted.
CREATE TRIGGER notify_user_update AFTER UPDATE ON users FOR EACH ROW EXECUTE FUNCTION notify_user();
CREATE TRIGGER notify_user_insert AFTER DELETE ON users FOR EACH ROW EXECUTE FUNCTION notify_user();