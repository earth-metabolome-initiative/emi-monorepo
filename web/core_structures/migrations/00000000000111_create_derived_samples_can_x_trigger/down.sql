-- Drop the `can_view_derived_samples` function and trigger on the derived_samples table.

DROP FUNCTION IF EXISTS can_view_derived_samples(INTEGER, UUID, UUID);
-- Drop the `can_admin_derived_samples` function and trigger on the derived_samples table.

DROP FUNCTION IF EXISTS can_admin_derived_samples(INTEGER, UUID, UUID);
-- Drop the `can_update_derived_samples` function and trigger on the derived_samples table.

DROP TRIGGER can_update_derived_samples ON derived_samples;
DROP FUNCTION IF EXISTS can_update_derived_samples_trigger();
DROP FUNCTION IF EXISTS can_update_derived_samples(INTEGER, UUID, UUID);
