-- Drop the `can_view_user_emails` function and trigger on the user_emails table.

DROP FUNCTION IF EXISTS can_view_user_emails(INTEGER, INTEGER);
-- Drop the `can_admin_user_emails` function and trigger on the user_emails table.

DROP FUNCTION IF EXISTS can_admin_user_emails(INTEGER, INTEGER);
