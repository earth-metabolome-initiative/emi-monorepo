-- Loads from the file kingdoms.csv the table kingdoms
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY kingdoms(name, description, font_awesome_icon, icon_color)
FROM '/app/kingdoms.csv' DELIMITER ',' CSV HEADER;
