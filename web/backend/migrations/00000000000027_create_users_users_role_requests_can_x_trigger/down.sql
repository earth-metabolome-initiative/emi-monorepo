-- Drop the `can_update_users_users_role_requests` function and trigger on the users_users_role_requests table.

DROP FUNCTION IF EXISTS can_update_users_users_role_requests(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_users_users_role_requests` function and trigger on the users_users_role_requests table.

DROP FUNCTION IF EXISTS can_admin_users_users_role_requests(INTEGER, INTEGER, INTEGER);
-- Drop the `can_view_users_users_role_requests` function and trigger on the users_users_role_requests table.

DROP FUNCTION IF EXISTS can_view_users_users_role_requests(INTEGER, INTEGER, INTEGER);
