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

-- now we want to assert that the number of lines in the table is the same as the number 
-- of lines in the temporary table
DO $$ DECLARE tmp_project_states_count INTEGER;

project_states_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_project_states_count
FROM
    tmp_project_states;

SELECT
    COUNT(*) INTO project_states_count
FROM
    project_states;

IF tmp_project_states_count <> project_states_count THEN RAISE EXCEPTION 'The number of rows in the temporary table is different from the number of rows in the table';

END IF;

END $$;

DROP TABLE tmp_project_states;