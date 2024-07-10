-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_permanence_categories(
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);
COPY tmp_permanence_categories
FROM
    '/app/permanence_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    permanence_categories(
        name,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_permanence_categories.name,
    tmp_permanence_categories.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_permanence_categories
    JOIN font_awesome_icons ON tmp_permanence_categories.icon = font_awesome_icons.name
    JOIN colors ON tmp_permanence_categories.color = colors.name;

-- now we want to assert that the number of lines in the sample_container_categories table is the same as the number
-- of lines in the tmp_sample_container_categories table
DO $$ DECLARE tmp_permanence_categories_count INTEGER;

permanence_categories_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_permanence_categories_count
FROM
    tmp_permanence_categories;

SELECT
    COUNT(*) INTO permanence_categories_count
FROM
    permanence_categories;

IF tmp_permanence_categories_count <> permanence_categories_count THEN RAISE EXCEPTION 'The number of rows in the tmp_permanence_categories table is different from the number of rows in the permanence_categories table';

END IF;
END $$;

DROP TABLE tmp_permanence_categories;