-- Drop the `can_edit_observations` function and trigger on the observations table.

DROP TRIGGER can_edit_observations ON observations;
DROP FUNCTION IF EXISTS can_edit_observations_trigger();
DROP FUNCTION IF EXISTS can_edit_observations(INTEGER, UUID);
-- Drop the `can_delete_observations` function and trigger on the observations table.

DROP FUNCTION IF EXISTS can_delete_observations(INTEGER, UUID);
-- Drop the `can_view_observations` function and trigger on the observations table.

DROP FUNCTION IF EXISTS can_view_observations(INTEGER, UUID);
