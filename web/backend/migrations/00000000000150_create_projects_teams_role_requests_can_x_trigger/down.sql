-- Drop the `can_view_projects_teams_role_requests` function and trigger on the projects_teams_role_requests table.

DROP FUNCTION IF EXISTS can_view_projects_teams_role_requests(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_projects_teams_role_requests` function and trigger on the projects_teams_role_requests table.

DROP FUNCTION IF EXISTS can_admin_projects_teams_role_requests(INTEGER, INTEGER, INTEGER);
