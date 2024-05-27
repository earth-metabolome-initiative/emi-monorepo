-- Drop the `can_update_samples` function and trigger on the samples table.

DROP TRIGGER can_update_samples ON samples;
DROP FUNCTION IF EXISTS can_update_samples_trigger();
DROP FUNCTION IF EXISTS can_update_samples(INTEGER, UUID);
-- Drop the `can_delete_samples` function and trigger on the samples table.

DROP FUNCTION IF EXISTS can_delete_samples(INTEGER, UUID);
-- Drop the `can_view_samples` function and trigger on the samples table.

DROP FUNCTION IF EXISTS can_view_samples(INTEGER, UUID);
