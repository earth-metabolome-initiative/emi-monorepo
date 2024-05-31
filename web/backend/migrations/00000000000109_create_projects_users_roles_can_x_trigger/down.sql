-- Drop the `can_update_projects_users_roles` function and trigger on the projects_users_roles table.

DROP FUNCTION IF EXISTS can_update_projects_users_roles(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_projects_users_roles` function and trigger on the projects_users_roles table.

DROP FUNCTION IF EXISTS can_admin_projects_users_roles(INTEGER, INTEGER, INTEGER);
-- Drop the `can_view_projects_users_roles` function and trigger on the projects_users_roles table.

DROP FUNCTION IF EXISTS can_view_projects_users_roles(INTEGER, INTEGER, INTEGER);
