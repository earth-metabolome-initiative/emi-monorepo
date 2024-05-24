-- Drop the `can_edit_teams` function and trigger on the teams table.

DROP TRIGGER can_edit_teams ON teams;
DROP FUNCTION IF EXISTS can_edit_teams_trigger();
DROP FUNCTION IF EXISTS can_edit_teams(INTEGER, INTEGER);
-- Drop the `can_delete_teams` function and trigger on the teams table.

DROP FUNCTION IF EXISTS can_delete_teams(INTEGER, INTEGER);
-- Drop the `can_view_teams` function and trigger on the teams table.

DROP FUNCTION IF EXISTS can_view_teams(INTEGER, INTEGER);
