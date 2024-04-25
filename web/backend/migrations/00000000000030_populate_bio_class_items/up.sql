-- Loads from the file bio_class_items.csv the table bio_class_items
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY bio_class_items(name, ott_id, ncbi_id, gbif_id, domain_id, kingdom_id, phylum_id)
FROM '/app/bio_class_items.csv' DELIMITER ',' CSV HEADER;
