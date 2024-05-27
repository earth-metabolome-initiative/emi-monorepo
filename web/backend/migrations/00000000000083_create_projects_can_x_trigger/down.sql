-- Drop the `can_update_projects` function and trigger on the projects table.

DROP TRIGGER can_update_projects ON projects;
DROP FUNCTION IF EXISTS can_update_projects_trigger();
DROP FUNCTION IF EXISTS can_update_projects(INTEGER, INTEGER);
-- Drop the `can_delete_projects` function and trigger on the projects table.

DROP FUNCTION IF EXISTS can_delete_projects(INTEGER, INTEGER);
-- Drop the `can_view_projects` function and trigger on the projects table.

DROP FUNCTION IF EXISTS can_view_projects(INTEGER, INTEGER);
