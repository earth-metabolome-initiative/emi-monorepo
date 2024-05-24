-- Drop the `can_edit_users` function and trigger on the users table.

DROP TRIGGER can_edit_users ON users;
DROP FUNCTION IF EXISTS can_edit_users_trigger();
DROP FUNCTION IF EXISTS can_edit_users(INTEGER, INTEGER);
-- Drop the `can_delete_users` function and trigger on the users table.

DROP FUNCTION IF EXISTS can_delete_users(INTEGER, INTEGER);
-- Drop the `can_view_users` function and trigger on the users table.

DROP FUNCTION IF EXISTS can_view_users(INTEGER, INTEGER);
