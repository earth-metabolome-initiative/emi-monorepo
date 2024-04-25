-- Loads from the file bio_phylum_items.csv the table bio_phylum_items
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY bio_phylum_items(name, ott_id, ncbi_id, gbif_id, domain_id, kingdom_id)
FROM '/app/bio_phylum_items.csv' DELIMITER ',' CSV HEADER;
