-- Drop the `can_view_projects_teams_roles` function and trigger on the projects_teams_roles table.

DROP FUNCTION IF EXISTS can_view_projects_teams_roles(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_projects_teams_roles` function and trigger on the projects_teams_roles table.

DROP FUNCTION IF EXISTS can_admin_projects_teams_roles(INTEGER, INTEGER, INTEGER);
