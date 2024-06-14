-- Drop the `can_update_organisms` function and trigger on the organisms table.

DROP TRIGGER can_update_organisms ON organisms;
DROP FUNCTION IF EXISTS can_update_organisms_trigger();
DROP FUNCTION IF EXISTS can_update_organisms(INTEGER, UUID);
-- Drop the `can_admin_organisms` function and trigger on the organisms table.

DROP FUNCTION IF EXISTS can_admin_organisms(INTEGER, UUID);
-- Drop the `can_view_organisms` function and trigger on the organisms table.

DROP FUNCTION IF EXISTS can_view_organisms(INTEGER, UUID);
