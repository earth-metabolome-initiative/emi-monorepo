-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_units(
    name TEXT NOT NULL,
    unit TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);

COPY tmp_units
FROM
    '/app/units.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    units(
        name,
        unit,
        icon_id,
        color_id
    )
SELECT
    tmp_units.name,
    tmp_units.unit,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_units
    JOIN font_awesome_icons ON tmp_units.icon = font_awesome_icons.name
    JOIN colors ON tmp_units.color = colors.name;

-- now we want to assert that the number of lines in the units table is the same as the number
-- of lines in the tmp_units table
DO $$ DECLARE tmp_units_count INTEGER;

units_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_units_count
FROM
    tmp_units;

SELECT
    COUNT(*) INTO units_count
FROM
    units;

IF tmp_units_count <> units_count THEN RAISE EXCEPTION 'The number of rows in the tmp_units table is different from the number of rows in the units table';

END IF;

END $$;

DROP TABLE tmp_units;

