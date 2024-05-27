-- Drop the `can_update_users_users_roles` function and trigger on the users_users_roles table.

DROP FUNCTION IF EXISTS can_update_users_users_roles(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_users_users_roles` function and trigger on the users_users_roles table.

DROP FUNCTION IF EXISTS can_admin_users_users_roles(INTEGER, INTEGER, INTEGER);
