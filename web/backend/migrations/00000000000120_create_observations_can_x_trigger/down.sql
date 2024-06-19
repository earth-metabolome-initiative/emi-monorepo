-- Drop the `can_view_observations` function and trigger on the observations table.

DROP FUNCTION IF EXISTS can_view_observations(INTEGER, UUID);
-- Drop the `can_admin_observations` function and trigger on the observations table.

DROP FUNCTION IF EXISTS can_admin_observations(INTEGER, UUID);
-- Drop the `can_update_observations` function and trigger on the observations table.

DROP TRIGGER can_update_observations ON observations;
DROP FUNCTION IF EXISTS can_update_observations_trigger();
DROP FUNCTION IF EXISTS can_update_observations(INTEGER, UUID);
