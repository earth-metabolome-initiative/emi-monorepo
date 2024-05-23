-- Drop the `can_edit_sampled_individual_bio_ott_taxon_items` function and trigger on the sampled_individual_bio_ott_taxon_items table.

DROP TRIGGER can_edit_sampled_individual_bio_ott_taxon_items ON sampled_individual_bio_ott_taxon_items;
DROP FUNCTION IF EXISTS can_edit_sampled_individual_bio_ott_taxon_items_trigger();
DROP FUNCTION IF EXISTS can_edit_sampled_individual_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
-- Drop the `can_delete_sampled_individual_bio_ott_taxon_items` function and trigger on the sampled_individual_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_delete_sampled_individual_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
-- Drop the `can_view_sampled_individual_bio_ott_taxon_items` function and trigger on the sampled_individual_bio_ott_taxon_items table.

DROP FUNCTION IF EXISTS can_view_sampled_individual_bio_ott_taxon_items(INTEGER, UUID, INTEGER);
