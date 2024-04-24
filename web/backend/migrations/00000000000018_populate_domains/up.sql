-- Loads from the file domains.csv the table organism_domains
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY organism_domains(name, description, font_awesome_icon, icon_color)
FROM '/app/domains.csv' DELIMITER ',' CSV HEADER;
