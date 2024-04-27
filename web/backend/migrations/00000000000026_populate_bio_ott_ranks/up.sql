-- Loads from the file bio_ott_ranks.csv the table bio_ott_ranks
--
-- The file has headers:
--     name,font_awesome_icon

CREATE TEMP TABLE tmp_bio_ott_ranks(
    name TEXT,
    font_awesome_icon TEXT
);

COPY tmp_bio_ott_ranks FROM '/app/bio_ott_ranks.csv' DELIMITER ',' CSV HEADER;

INSERT INTO bio_ott_ranks(name, font_awesome_icon_id)
SELECT tmp_bio_ott_ranks.name, font_awesome_icons.id
FROM tmp_bio_ott_ranks JOIN font_awesome_icons ON tmp_bio_ott_ranks.font_awesome_icon = font_awesome_icons.name;

DROP TABLE tmp_bio_ott_ranks;