-- Loads from the file team_states.csv the table team_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color

COPY team_states(name, description, font_awesome_icon, icon_color)
FROM '/app/team_states.csv' DELIMITER ',' CSV HEADER;
