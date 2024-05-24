-- Drop the `can_edit_projects` function and trigger on the projects table.

DROP TRIGGER can_edit_projects ON projects;
DROP FUNCTION IF EXISTS can_edit_projects_trigger();
DROP FUNCTION IF EXISTS can_edit_projects(INTEGER, INTEGER);
-- Drop the `can_delete_projects` function and trigger on the projects table.

DROP FUNCTION IF EXISTS can_delete_projects(INTEGER, INTEGER);
-- Drop the `can_view_projects` function and trigger on the projects table.

DROP FUNCTION IF EXISTS can_view_projects(INTEGER, INTEGER);
