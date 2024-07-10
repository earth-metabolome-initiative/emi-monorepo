-- Drop the `can_view_organism_taxa` function and trigger on the organism_taxa table.

DROP FUNCTION IF EXISTS can_view_organism_taxa(INTEGER, UUID, INTEGER);
-- Drop the `can_admin_organism_taxa` function and trigger on the organism_taxa table.

DROP FUNCTION IF EXISTS can_admin_organism_taxa(INTEGER, UUID, INTEGER);
