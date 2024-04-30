-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notifications (
    id SERIAL PRIMARY KEY,
    -- The user to whom the notification is addressed
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- An operation such as "INSERT", "UPDATE", or "DELETE"
    operation VARCHAR(6) NOT NULL,
    -- The table on which the operation was performed
    table_name VARCHAR(255) NOT NULL,
    -- Whether the notification has been read
    read BOOLEAN NOT NULL DEFAULT FALSE,
    CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE'))
);

-- We create a function that notifies the user when a row is
-- inserted, edited or deleted in the notifications table.
CREATE FUNCTION notify_user()
RETURNS TRIGGER AS $$
DECLARE
    serialized_record TEXT;
    notification_record notifications;
    user_id INTEGER;
BEGIN
    if TG_OP = 'INSERT' THEN
        user_id := NEW.id;
    ELSE
        user_id := OLD.id;
    END IF;

    -- We serialize the ROW object into a JSON string
    -- and afterwards we will combine it with the newly inserted
    -- notification record to send it to the user.
    -- Depending on whether the operation is a Deletion or not,
    -- we use the OLD or NEW object to serialize the record.
    IF TG_OP = 'DELETE' THEN
        serialized_record := row_to_json(OLD)::TEXT;
    ELSE
        serialized_record := row_to_json(NEW)::TEXT;
    END IF;

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
CREATE TRIGGER users_notify_trigger
AFTER UPDATE OR DELETE
ON users
FOR EACH ROW
EXECUTE FUNCTION notify_user();

