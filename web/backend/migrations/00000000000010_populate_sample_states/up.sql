-- Loads from the file sample_states.csv the table sample_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY sample_states(name, description, font_awesome_icon, icon_color)
FROM '/app/sample_states.csv' DELIMITER ',' CSV HEADER;
