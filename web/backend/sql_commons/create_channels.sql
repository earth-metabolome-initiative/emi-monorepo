-- Create a trigger function to notify a channel when a new comment is inserted
CREATE
OR REPLACE FUNCTION notify() RETURNS TRIGGER AS $$
DECLARE
    id INTEGER;

BEGIN
    IF TG_OP = 'INSERT'
    OR TG_OP = 'UPDATE' THEN id = NEW .id;

ELSE id = OLD .id;

END IF;

PERFORM pg_notify(
    LOWER(TG_TABLE_NAME) || '_' || LOWER(TG_OP) || '_' || id,
    json_build_object('id', id) :: text
);

RETURN NEW;

END;

$$ LANGUAGE plpgsql;

CREATE
OR REPLACE FUNCTION create_dynamic_trigger(
    table_name TEXT,
    operation TEXT,
    -- INSERT, UPDATE, DELETE
    trigger_function TEXT
) RETURNS VOID AS $$
DECLARE
    trigger_name TEXT;

BEGIN
    -- Generate a unique trigger name dynamically based on some criteria
    trigger_name := 'dynamic_trigger' || '_' || table_name || '_' || LOWER(operation);

-- Execute dynamic SQL to create the trigger
EXECUTE format(
    'CREATE TRIGGER %I AFTER %i ON %I FOR EACH ROW EXECUTE PROCEDURE %I()',
    trigger_name,
    table_name,
    operation,
    trigger_function
);

END;

$$ LANGUAGE plpgsql;

CREATE
OR REPLACE FUNCTION create_notify_triggers(table_name TEXT,) RETURNS VOID AS $$
DECLARE
    trigger_name TEXT;

BEGIN
    -- Call the create_dynamic_trigger function to create the trigger
    PERFORM create_dynamic_trigger(table_name, 'INSERT', 'notify');

PERFORM create_dynamic_trigger(table_name, 'UPDATE', 'notify');

PERFORM create_dynamic_trigger(table_name, 'DELETE', 'notify');

END;

$$ LANGUAGE plpgsql;