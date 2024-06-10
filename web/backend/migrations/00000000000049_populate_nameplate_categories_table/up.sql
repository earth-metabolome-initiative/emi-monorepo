-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_nameplate_categories(
    name TEXT NOT NULL,
    material TEXT NOT NULL,
    permanence TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);

COPY tmp_nameplate_categories
FROM
    '/app/nameplate_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    nameplate_categories(
        name,
        permanence_id,
        material_id,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_nameplate_categories.name,
    permanence_categories.id,
    materials.id,
    tmp_nameplate_categories.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_nameplate_categories
    JOIN font_awesome_icons ON tmp_nameplate_categories.icon = font_awesome_icons.name
    JOIN permanence_categories ON tmp_nameplate_categories.permanence = permanence_categories.name
    JOIN colors ON tmp_nameplate_categories.color = colors.name
    JOIN materials ON tmp_nameplate_categories.material = materials.name;

-- now we want to assert that the number of lines in the nameplate_categories table is the same as the number
-- of lines in the tmp_nameplate_categories table
DO $$ DECLARE tmp_nameplate_categories_count INTEGER;

nameplate_categories_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_nameplate_categories_count
FROM
    tmp_nameplate_categories;

SELECT
    COUNT(*) INTO nameplate_categories_count
FROM
    nameplate_categories;

IF tmp_nameplate_categories_count <> nameplate_categories_count THEN RAISE EXCEPTION 'The number of rows in the tmp_nameplate_categories table is different from the number of rows in the nameplate_categories table';

END IF;

END $$;

DROP TABLE tmp_nameplate_categories;

