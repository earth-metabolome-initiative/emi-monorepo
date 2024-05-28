-- Drop the `can_update_sampled_individuals` function and trigger on the sampled_individuals table.

DROP TRIGGER can_update_sampled_individuals ON sampled_individuals;
DROP FUNCTION IF EXISTS can_update_sampled_individuals_trigger();
DROP FUNCTION IF EXISTS can_update_sampled_individuals(INTEGER, UUID);
-- Drop the `can_admin_sampled_individuals` function and trigger on the sampled_individuals table.

DROP FUNCTION IF EXISTS can_admin_sampled_individuals(INTEGER, UUID);
-- Drop the `can_view_sampled_individuals` function and trigger on the sampled_individuals table.

DROP FUNCTION IF EXISTS can_view_sampled_individuals(INTEGER, UUID);
