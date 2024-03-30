-- Loads from the file project_states.csv the table project_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY project_states(name, description, font_awesome_icon, icon_color)
FROM '/app/project_states.csv' DELIMITER ',' CSV HEADER;
