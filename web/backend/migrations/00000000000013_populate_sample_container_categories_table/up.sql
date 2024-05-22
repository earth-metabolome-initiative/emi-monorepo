-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_sample_container_categories(
    brand TEXT NOT NULL,
    volume INTEGER NOT NULL,
    description TEXT NOT NULL UNIQUE,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);

COPY tmp_sample_container_categories
FROM
    '/app/sample_container_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    sample_container_categories(
        brand,
        volume,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_sample_container_categories.brand,
    tmp_sample_container_categories.volume,
    tmp_sample_container_categories.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_sample_container_categories
    JOIN font_awesome_icons ON tmp_sample_container_categories.icon = font_awesome_icons.name
    JOIN colors ON tmp_sample_container_categories.color = colors.name;

-- now we want to assert that the number of lines in the sample_container_categories table is the same as the number
-- of lines in the tmp_sample_container_categories table
DO $$ DECLARE tmp_sample_container_categories_count INTEGER;

sample_container_categories_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_sample_container_categories_count
FROM
    tmp_sample_container_categories;

SELECT
    COUNT(*) INTO sample_container_categories_count
FROM
    sample_container_categories;

IF tmp_sample_container_categories_count <> sample_container_categories_count THEN RAISE EXCEPTION 'The number of rows in the tmp_sample_container_categories table is different from the number of rows in the sample_container_categories table';

END IF;

END $$;

DROP TABLE tmp_sample_container_categories;

