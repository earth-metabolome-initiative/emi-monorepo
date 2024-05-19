-- Loads from the file team_states.csv the table team_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color
CREATE TEMPORARY TABLE tmp_team_states(
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
        icon_id,
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

-- now we want to assert that the number of lines in the table is the same as the number 
-- of lines in the temporary table
DO $$ DECLARE tmp_team_states_count INTEGER;

team_states_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_team_states_count
FROM
    tmp_team_states;

SELECT
    COUNT(*) INTO team_states_count
FROM
    team_states;

IF tmp_team_states_count <> team_states_count THEN RAISE EXCEPTION 'The number of rows in the temporary table is different from the number of rows in the table';

END IF;

END $$;

DROP TABLE tmp_team_states;