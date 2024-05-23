-- Drop the `can_edit_sample_containers` function and trigger on the sample_containers table.

DROP FUNCTION IF EXISTS can_edit_sample_containers(INTEGER, INTEGER);
-- Drop the `can_delete_sample_containers` function and trigger on the sample_containers table.

DROP FUNCTION IF EXISTS can_delete_sample_containers(INTEGER, INTEGER);
-- Drop the `can_view_sample_containers` function and trigger on the sample_containers table.

DROP FUNCTION IF EXISTS can_view_sample_containers(INTEGER, INTEGER);
