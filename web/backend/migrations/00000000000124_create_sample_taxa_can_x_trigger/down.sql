-- Drop the `can_view_sample_taxa` function and trigger on the sample_taxa table.

DROP FUNCTION IF EXISTS can_view_sample_taxa(INTEGER, UUID, INTEGER);
-- Drop the `can_admin_sample_taxa` function and trigger on the sample_taxa table.

DROP FUNCTION IF EXISTS can_admin_sample_taxa(INTEGER, UUID, INTEGER);
