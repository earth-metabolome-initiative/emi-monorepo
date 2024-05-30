-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_observation_subjects(
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);

COPY tmp_observation_subjects
FROM
    '/app/observation_subjects.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    observation_subjects(
        name,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_observation_subjects.name,
    tmp_observation_subjects.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_observation_subjects
    JOIN font_awesome_icons ON tmp_observation_subjects.icon = font_awesome_icons.name
    JOIN colors ON tmp_observation_subjects.color = colors.name;

-- now we want to assert that the number of lines in the observation_subjects table is the same as the number
-- of lines in the tmp_observation_subjects table
DO $$ DECLARE tmp_observation_subjects_count INTEGER;

observation_subjects_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_observation_subjects_count
FROM
    tmp_observation_subjects;

SELECT
    COUNT(*) INTO observation_subjects_count
FROM
    observation_subjects;

IF tmp_observation_subjects_count <> observation_subjects_count THEN RAISE EXCEPTION 'The number of rows in the tmp_observation_subjects table is different from the number of rows in the observation_subjects table';

END IF;

END $$;

DROP TABLE tmp_observation_subjects;

