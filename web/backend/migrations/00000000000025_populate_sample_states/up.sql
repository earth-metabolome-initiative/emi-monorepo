-- Loads from the file sample_states.csv the table sample_states
--
-- The file has headers:
-- name,description,font_awesome_icon,icon_color
CREATE TEMPORARY TABLE tmp_sample_states(
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

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_sample_states_count INTEGER;

sample_states_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_sample_states_count
FROM
    tmp_sample_states;

SELECT
    COUNT(*) INTO sample_states_count
FROM
    sample_states;

IF tmp_sample_states_count <> sample_states_count THEN RAISE EXCEPTION 'The number of rows in the tmp_sample_states table is different from the number of rows in the sample_states table';

END IF;

END $$;

DROP TABLE tmp_sample_states;