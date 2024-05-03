-- Loads from the file team_states.csv the table team_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color
CREATE TEMP TABLE tmp_team_states(
    name TEXT,
    description TEXT,
    font_awesome_icon TEXT,
    icon_color TEXT
);

COPY tmp_team_states
FROM
    '/app/team_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    team_states(
        name,
        description,
        font_awesome_icon_id,
        color_id
    )
SELECT
    tmp_team_states.name,
    tmp_team_states.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_team_states
    JOIN font_awesome_icons ON tmp_team_states.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_team_states.icon_color = colors.name;

DROP TABLE tmp_team_states;