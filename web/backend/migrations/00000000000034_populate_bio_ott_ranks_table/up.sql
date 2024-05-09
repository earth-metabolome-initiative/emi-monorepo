-- Loads from the file bio_ott_ranks.csv the table bio_ott_ranks
--
-- The file has headers:
--     name,font_awesome_icon
CREATE TEMPORARY TABLE tmp_bio_ott_ranks(name TEXT, font_awesome_icon TEXT);

COPY tmp_bio_ott_ranks
FROM
    '/app/bio_ott_ranks.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    bio_ott_ranks(name, font_awesome_icon_id)
SELECT
    tmp_bio_ott_ranks.name,
    font_awesome_icons.id
FROM
    tmp_bio_ott_ranks
    JOIN font_awesome_icons ON tmp_bio_ott_ranks.font_awesome_icon = font_awesome_icons.name;

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

IF tmp_bio_ott_ranks_count <> bio_ott_ranks_count THEN RAISE EXCEPTION 'The number of rows in the tmp_bio_ott_ranks table is different from the number of rows in the bio_ott_ranks table';

END IF;

END $$;

DROP TABLE tmp_bio_ott_ranks;