-- We create a function that notifies the user who is authoring
-- a project when a new project is created. This function will
-- send a notification through the pg_notify function to the
-- user's channel with the newly created project, plus will also
-- insert a new row in the notifications table to keep track of
-- the operation that was performed.
CREATE FUNCTION notify_user_for_project_creation()
RETURNS TRIGGER AS $$
DECLARE
    serialized_record TEXT;
    notification_record notifications;
    user_id INTEGER;
BEGIN
    user_id := NEW.created_by;
    serialized_record := row_to_json(NEW)::TEXT;

    -- We insert a new row in the notifications table
    INSERT INTO notifications (user_id, operation, table_name)
    VALUES (user_id, TG_OP, TG_TABLE_NAME)
    RETURNING * INTO notification_record;

    -- We retrieve the record of the notification we have
    -- just inserted and serialize it into a JSON string

    PERFORM pg_notify(
        'user' || '_' || user_id,
        json_build_object(
            'notification', row_to_json(notification_record),
            'record', serialized_record
        )::TEXT
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- We define a trigger that calls the notify_user function
-- when an operation is performed on the users table to notify
-- the user of the modified row.
CREATE TRIGGER notify_user_for_project_creation_trigger
AFTER INSERT
ON projects
FOR EACH ROW
EXECUTE FUNCTION notify_user_for_project_creation();
