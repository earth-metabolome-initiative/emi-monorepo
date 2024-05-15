CREATE FUNCTION notify_user_for_project_creation()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO notifications (user_id, operation, table_name, record)
    VALUES (NEW.created_by, TG_OP, TG_TABLE_NAME, row_to_json(NEW)::TEXT);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notify_user_for_project_creation_trigger
AFTER INSERT
ON projects
FOR EACH ROW
EXECUTE FUNCTION notify_user_for_project_creation();
