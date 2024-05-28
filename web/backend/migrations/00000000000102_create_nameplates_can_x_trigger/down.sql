-- Drop the `can_update_nameplates` function and trigger on the nameplates table.

DROP TRIGGER can_update_nameplates ON nameplates;
DROP FUNCTION IF EXISTS can_update_nameplates_trigger();
DROP FUNCTION IF EXISTS can_update_nameplates(INTEGER, INTEGER);
-- Drop the `can_admin_nameplates` function and trigger on the nameplates table.

DROP FUNCTION IF EXISTS can_admin_nameplates(INTEGER, INTEGER);
-- Drop the `can_view_nameplates` function and trigger on the nameplates table.

DROP FUNCTION IF EXISTS can_view_nameplates(INTEGER, INTEGER);
