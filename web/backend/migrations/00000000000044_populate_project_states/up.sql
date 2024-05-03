-- Loads from the file project_states.csv the table project_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color
CREATE TEMPORARY TABLE tmp_project_states(
    name TEXT,
    description TEXT,
    font_awesome_icon TEXT,
    icon_color TEXT
);

COPY tmp_project_states
FROM
    '/app/project_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    project_states(
        name,
        description,
        font_awesome_icon_id,
        color_id
    )
SELECT
    tmp_project_states.name,
    tmp_project_states.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_project_states
    JOIN font_awesome_icons ON tmp_project_states.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_project_states.icon_color = colors.name;

DROP TABLE tmp_project_states;