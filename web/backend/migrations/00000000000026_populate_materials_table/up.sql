-- This is a no-op SQL statement
CREATE TEMPORARY TABLE tmp_materials(
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL
);
COPY tmp_materials
FROM
    '/app/materials.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    materials(
        name,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_materials.name,
    tmp_materials.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_materials
    JOIN font_awesome_icons ON tmp_materials.icon = font_awesome_icons.name
    JOIN colors ON tmp_materials.color = colors.name;

-- now we want to assert that the number of lines in the sample_container_categories table is the same as the number
-- of lines in the tmp_sample_container_categories table
DO $$ DECLARE tmp_materials_count INTEGER;

materials_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_materials_count
FROM
    tmp_materials;

SELECT
    COUNT(*) INTO materials_count
FROM
    materials;

IF tmp_materials_count <> materials_count THEN RAISE EXCEPTION 'The number of rows in the tmp_materials table is different from the number of rows in the materials table';

END IF;
END $$;

DROP TABLE tmp_materials;