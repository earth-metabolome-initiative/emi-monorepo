-- Drop the `can_update_sample_containers` function and trigger on the sample_containers table.

DROP TRIGGER can_update_sample_containers ON sample_containers;
DROP FUNCTION IF EXISTS can_update_sample_containers_trigger();
DROP FUNCTION IF EXISTS can_update_sample_containers(INTEGER, INTEGER);
-- Drop the `can_admin_sample_containers` function and trigger on the sample_containers table.

DROP FUNCTION IF EXISTS can_admin_sample_containers(INTEGER, INTEGER);
-- Drop the `can_view_sample_containers` function and trigger on the sample_containers table.

DROP FUNCTION IF EXISTS can_view_sample_containers(INTEGER, INTEGER);