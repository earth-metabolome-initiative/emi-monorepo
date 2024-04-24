-- Loads from the file phylums.csv the table phylums
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY phylums(name)
FROM '/app/phylums.csv' DELIMITER ',' CSV HEADER;
