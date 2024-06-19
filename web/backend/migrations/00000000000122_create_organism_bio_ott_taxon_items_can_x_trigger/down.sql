-- Drop the `can_view_organism_bio_ott_taxon_items` function and trigger on the organism_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_view_organism_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
-- Drop the `can_admin_organism_bio_ott_taxon_items` function and trigger on the organism_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_admin_organism_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
