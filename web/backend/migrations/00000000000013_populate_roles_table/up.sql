CREATE TEMPORARY TABLE tmp_roles(
    name TEXT,
    description TEXT,
    font_awesome_icon TEXT,
    color TEXT
);

COPY tmp_roles
FROM
    '/app/roles.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    roles(
        name,
        description,
        font_awesome_icon_id,
        color_id
    )
SELECT
    tmp_roles.name,
    tmp_roles.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_roles
    JOIN font_awesome_icons ON tmp_roles.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_roles.color = colors.name;

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_roles_count INTEGER;

roles_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_roles_count
FROM
    tmp_roles;

SELECT
    COUNT(*) INTO roles_count
FROM
    roles;

IF tmp_roles_count <> roles_count THEN RAISE EXCEPTION 'The number of rows in the tmp_roles table is different from the number of rows in the roles table';

END IF;

END $$;

DROP TABLE tmp_roles;