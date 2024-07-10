-- Loads from the file ranks.csv the table ranks
CREATE TEMPORARY TABLE tmp_ranks(
    name TEXT,
    font_awesome_icon TEXT,
    color TEXT,
    description TEXT
);

COPY tmp_ranks
FROM
    '/app/ranks.csv' DELIMITER ',' CSV HEADER;


-- We check that all of the colors in the tmp_ranks table are present in the colors table
DO $$ DECLARE color_name TEXT;

BEGIN
FOR color_name IN SELECT DISTINCT color FROM tmp_ranks LOOP
    IF NOT EXISTS(SELECT 1 FROM colors WHERE name = color_name) THEN RAISE EXCEPTION 'The color % does not exist in the colors table', color_name;

    END IF;

END LOOP;

END $$;

-- We check that all of the font_awesome_icons in the tmp_ranks table are present in the font_awesome_icons table
DO $$ DECLARE font_awesome_icon_name TEXT;

BEGIN
FOR font_awesome_icon_name IN SELECT DISTINCT font_awesome_icon FROM tmp_ranks LOOP
    IF NOT EXISTS(SELECT 1 FROM font_awesome_icons WHERE name = font_awesome_icon_name) THEN RAISE EXCEPTION 'The font_awesome_icon % does not exist in the font_awesome_icons table', font_awesome_icon_name;

    END IF;

END LOOP;

END $$;


INSERT INTO
    ranks(
        name,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_ranks.name,
    tmp_ranks.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_ranks
    JOIN font_awesome_icons ON tmp_ranks.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_ranks.color = colors.name;

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_ranks_count INTEGER;

ranks_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_ranks_count
FROM
    tmp_ranks;

SELECT
    COUNT(*) INTO ranks_count
FROM
    ranks;

IF tmp_ranks_count <> ranks_count THEN RAISE EXCEPTION 'The number of rows in the tmp_ranks table (%1) is different from the number of rows in the ranks table (%2)', tmp_ranks_count, ranks_count;

END IF;

END $$;

DROP TABLE tmp_ranks;