-- Loads from the file classes.csv the table classes
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY classes(name)
FROM '/app/classes.csv' DELIMITER ',' CSV HEADER;
