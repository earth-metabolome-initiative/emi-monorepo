-- We drop the trigger and the function that notifies the user
DROP TRIGGER notify_user_for_project_creation_trigger ON projects;
DROP FUNCTION notify_user_for_project_creation();