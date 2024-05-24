-- Drop the `can_edit_sampled_individuals` function and trigger on the sampled_individuals table.

DROP TRIGGER can_edit_sampled_individuals ON sampled_individuals;
DROP FUNCTION IF EXISTS can_edit_sampled_individuals_trigger();
DROP FUNCTION IF EXISTS can_edit_sampled_individuals(INTEGER, UUID);
-- Drop the `can_delete_sampled_individuals` function and trigger on the sampled_individuals table.

DROP FUNCTION IF EXISTS can_delete_sampled_individuals(INTEGER, UUID);
-- Drop the `can_view_sampled_individuals` function and trigger on the sampled_individuals table.

DROP FUNCTION IF EXISTS can_view_sampled_individuals(INTEGER, UUID);
