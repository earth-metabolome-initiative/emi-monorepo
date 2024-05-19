-- Loads from the file bio_ott_ranks.csv the table bio_ott_ranks
CREATE TEMPORARY TABLE tmp_bio_ott_ranks(
    name TEXT,
    font_awesome_icon TEXT,
    color TEXT,
    description TEXT
);

COPY tmp_bio_ott_ranks
FROM
    '/app/bio_ott_ranks.csv' DELIMITER ',' CSV HEADER;


-- We check that all of the colors in the tmp_bio_ott_ranks table are present in the colors table
DO $$ DECLARE color_name TEXT;

BEGIN
FOR color_name IN SELECT DISTINCT color FROM tmp_bio_ott_ranks LOOP
    IF NOT EXISTS(SELECT 1 FROM colors WHERE name = color_name) THEN RAISE EXCEPTION 'The color % does not exist in the colors table', color_name;

    END IF;

END LOOP;

END $$;

-- We check that all of the font_awesome_icons in the tmp_bio_ott_ranks table are present in the font_awesome_icons table
DO $$ DECLARE font_awesome_icon_name TEXT;

BEGIN
FOR font_awesome_icon_name IN SELECT DISTINCT font_awesome_icon FROM tmp_bio_ott_ranks LOOP
    IF NOT EXISTS(SELECT 1 FROM font_awesome_icons WHERE name = font_awesome_icon_name) THEN RAISE EXCEPTION 'The font_awesome_icon % does not exist in the font_awesome_icons table', font_awesome_icon_name;

    END IF;

END LOOP;

END $$;


INSERT INTO
    bio_ott_ranks(
        name,
        description,
        icon_id,
        color_id
    )
SELECT
    tmp_bio_ott_ranks.name,
    tmp_bio_ott_ranks.description,
    font_awesome_icons.id,
    colors.id
FROM
    tmp_bio_ott_ranks
    JOIN font_awesome_icons ON tmp_bio_ott_ranks.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_bio_ott_ranks.color = colors.name;

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_bio_ott_ranks_count INTEGER;

bio_ott_ranks_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_bio_ott_ranks_count
FROM
    tmp_bio_ott_ranks;

SELECT
    COUNT(*) INTO bio_ott_ranks_count
FROM
    bio_ott_ranks;

IF tmp_bio_ott_ranks_count <> bio_ott_ranks_count THEN RAISE EXCEPTION 'The number of rows in the tmp_bio_ott_ranks table (%1) is different from the number of rows in the bio_ott_ranks table (%2)', tmp_bio_ott_ranks_count, bio_ott_ranks_count;

END IF;

END $$;

DROP TABLE tmp_bio_ott_ranks;