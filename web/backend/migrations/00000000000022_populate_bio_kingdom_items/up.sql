-- Loads from the file bio_kingdom_items.csv the table bio_kingdom_items
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY bio_kingdom_items(name, ott_id, ncbi_id, gbif_id, domain_id, description, font_awesome_icon, icon_color)
FROM '/app/bio_kingdom_items.csv' DELIMITER ',' CSV HEADER;

