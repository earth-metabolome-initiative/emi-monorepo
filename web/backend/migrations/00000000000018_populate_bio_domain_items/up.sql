-- Loads from the file bio_domains.csv the table bio_domain_items
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY bio_domain_items(name, ott_id, ncbi_id, gbif_id, description, font_awesome_icon, icon_color)
FROM '/app/bio_domain_items.csv' DELIMITER ',' CSV HEADER;
