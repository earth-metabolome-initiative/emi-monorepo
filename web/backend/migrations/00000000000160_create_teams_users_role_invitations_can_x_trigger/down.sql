-- Drop the `can_view_teams_users_role_invitations` function and trigger on the teams_users_role_invitations table.

DROP FUNCTION IF EXISTS can_view_teams_users_role_invitations(INTEGER, INTEGER, INTEGER);
-- Drop the `can_admin_teams_users_role_invitations` function and trigger on the teams_users_role_invitations table.

DROP FUNCTION IF EXISTS can_admin_teams_users_role_invitations(INTEGER, INTEGER, INTEGER);
