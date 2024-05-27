-- Drop the `can_update_sample_bio_ott_taxon_items` function and trigger on the sample_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_update_sample_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
-- Drop the `can_admin_sample_bio_ott_taxon_items` function and trigger on the sample_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_admin_sample_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
-- Drop the `can_view_sample_bio_ott_taxon_items` function and trigger on the sample_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_view_sample_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
