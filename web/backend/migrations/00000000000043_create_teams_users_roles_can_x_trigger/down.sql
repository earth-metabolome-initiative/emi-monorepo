-- Drop the `can_update_teams_users_roles` function and trigger on the teams_users_roles table.

DROP FUNCTION IF EXISTS can_update_teams_users_roles(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_teams_users_roles` function and trigger on the teams_users_roles table.

DROP FUNCTION IF EXISTS can_admin_teams_users_roles(INTEGER, INTEGER, INTEGER);
