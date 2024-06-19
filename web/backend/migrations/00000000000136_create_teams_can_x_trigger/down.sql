-- Drop the `can_admin_teams` function and trigger on the teams table.

DROP FUNCTION IF EXISTS can_admin_teams(INTEGER, INTEGER);
-- Drop the `can_update_teams` function and trigger on the teams table.

DROP TRIGGER can_update_teams ON teams;
DROP FUNCTION IF EXISTS can_update_teams_trigger();
DROP FUNCTION IF EXISTS can_update_teams(INTEGER, INTEGER);
