-- Drop the `can_update_users` function and trigger on the users table.

DROP TRIGGER can_update_users ON users;
DROP FUNCTION IF EXISTS can_update_users_trigger();
DROP FUNCTION IF EXISTS can_update_users(INTEGER, INTEGER);
-- Drop the `can_admin_users` function and trigger on the users table.

DROP FUNCTION IF EXISTS can_admin_users(INTEGER, INTEGER);
