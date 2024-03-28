-- Your SQL goes here
CREATE TABLE notifications (
    id UUID PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
    -- The user to whom the notification is addressed
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- An operation such as "INSERT", "UPDATE", or "DELETE"
    operation VARCHAR(6) NOT NULL,
    -- The table on which the operation was performed
    table_name VARCHAR(255) NOT NULL,
    -- The ID of the row that was affected
    row_id UUID REFERENCES editables (id) ON DELETE SET NULL,
    -- Whether the notification has been read
    read BOOLEAN NOT NULL DEFAULT FALSE,
    CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE'))
);

-- We create a function that notifies the user when a row is
-- inserted, edited or deleted in the notifications table.
CREATE OR REPLACE FUNCTION notify_user()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify(
        'user' || '_' || NEW.user_id,
        json_build_object(
            'operation', TG_OP,
            'notification', row_to_json(NEW)::TEXT
        )::TEXT
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- We create a trigger that notifies the user when a row is
-- inserted, edited or deleted in the notifications table.
CREATE TRIGGER notify_user
AFTER INSERT OR UPDATE OR DELETE ON notifications
FOR EACH ROW
EXECUTE FUNCTION notify_user();