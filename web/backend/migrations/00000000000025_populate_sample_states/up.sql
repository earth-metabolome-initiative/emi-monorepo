-- Loads from the file sample_states.csv the table sample_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color
CREATE TEMP TABLE tmp_sample_states(
    name TEXT,
    description TEXT,
    font_awesome_icon TEXT,
    icon_color TEXT
);

COPY tmp_sample_states
FROM
    '/app/sample_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    sample_states(
        name,
        description,
        font_awesome_icon_id,
        color_id
    )
SELECT
    tmp_sample_states.name,
    tmp_sample_states.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_sample_states
    JOIN font_awesome_icons ON tmp_sample_states.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_sample_states.icon_color = colors.name;

DROP TABLE tmp_sample_states;