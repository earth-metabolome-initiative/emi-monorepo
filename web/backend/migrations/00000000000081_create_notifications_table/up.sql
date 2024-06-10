-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY,
    -- The user to whom the notification is addressed
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- An operation such as "INSERT", "UPDATE", or "DELETE"
    operation text NOT NULL,
    -- The table on which the operation was performed
    table_name text NOT NULL,
    -- The JSON serialized data of the row that was inserted, updated, or deleted
    record TEXT NOT NULL,
    -- Whether the notification has been read
    read BOOLEAN NOT NULL DEFAULT FALSE,
    CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE'))
);

-- Function to be called when a notification is inserted or updated
-- in the notifications table to call a pg_notify function to notify the user
-- with the serialized notification record.
CREATE FUNCTION custom_notification_change()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify(
        'user' || '_' || NEW.user_id,
        row_to_json(NEW)::TEXT
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- We define a trigger that calls the pg_notify function
-- every time a notification is inserted or updated into the notifications
-- table to notify the user of the new notification.
DROP TRIGGER IF EXISTS notifications_notify_trigger ON notifications;
CREATE TRIGGER notifications_notify_trigger
AFTER INSERT OR UPDATE
ON notifications
FOR EACH ROW
EXECUTE FUNCTION custom_notification_change();

-- -- We create a function that notifies the user when a row is
-- -- inserted, edited or deleted in the notifications table.
-- CREATE FUNCTION creation_user_notification()
-- RETURNS TRIGGER AS $$
-- DECLARE
--     serialized_record TEXT;
--     created_by INTEGER;
-- BEGIN
--     if TG_OP = 'INSERT' THEN
--         user_id := NEW.id;
--     ELSE
--         user_id := OLD.id;
--     END IF;

--     -- We serialize the ROW object into a JSON string
--     -- and afterwards we will combine it with the newly inserted
--     -- notification record to send it to the user.
--     -- Depending on whether the operation is a Deletion or not,
--     -- we use the OLD or NEW object to serialize the record.
--     IF TG_OP = 'DELETE' THEN
--         serialized_record := row_to_json(OLD)::TEXT;
--     ELSE
--         serialized_record := row_to_json(NEW)::TEXT;
--     END IF;

--     -- We insert a new row in the notifications table
--     INSERT INTO notifications (user_id, operation, table_name, record)
--     VALUES (user_id, TG_OP, TG_TABLE_NAME, serialized_record)
--     RETURNING * INTO notification_record;

--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;