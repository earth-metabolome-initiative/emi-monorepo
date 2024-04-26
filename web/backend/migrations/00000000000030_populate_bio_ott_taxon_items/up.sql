-- Loads from the file bio_ott_taxon_items.csv the table bio_ott_taxon_items
--
-- The file has headers:
--     name,description,ott_id, wikidata_id, ncbi_id, gbif_id, parent_id, font_awesome_icon, icon_color

CREATE TEMP TABLE tmp_bio_ott_taxon_items(
    uid INTEGER NOT NULL,
    parent_uid INTEGER DEFAULT 805080, -- 805080 is the uid of "Life" in Open Tree of Life
    name TEXT NOT NULL,
    rank TEXT NOT NULL,
    ncbi INTEGER,
    gbif INTEGER,
    irmng INTEGER,
    worms INTEGER,
    bio_kingdom TEXT,
    bio_phylum TEXT,
    bio_class TEXT,
    bio_order TEXT,
    bio_family TEXT,
    bio_genus TEXT,
    bio_domain TEXT,
    font_awesome_icon TEXT,
    icon_color TEXT,
    wikidata_id INTEGER
);
COPY tmp_bio_ott_taxon_items FROM PROGRAM 'gzip -dc /app/bio_ott_taxons.csv.gz' DELIMITER ',' CSV HEADER;

INSERT INTO bio_ott_taxon_items(name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, font_awesome_icon_id, color_id)
SELECT tmp_bio_ott_taxon_items.name, tmp_bio_ott_taxon_items.uid, bio_ott_ranks.id, tmp_bio_ott_taxon_items.wikidata_id, tmp_bio_ott_taxon_items.ncbi, tmp_bio_ott_taxon_items.gbif, font_awesome_icons.id, colors.id
FROM tmp_bio_ott_taxon_items JOIN bio_ott_ranks ON tmp_bio_ott_taxon_items.rank = bio_ott_ranks.name JOIN font_awesome_icons ON tmp_bio_ott_taxon_items.font_awesome_icon = font_awesome_icons.name JOIN colors ON tmp_bio_ott_taxon_items.icon_color = colors.name;

-- Once we have inserted the taxon items, we need to update the parent_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, domain_id

UPDATE bio_ott_taxon_items
SET parent_id = parent_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS parent_id_table ON tmp_bio_ott_taxon_items.parent_uid = parent_id_table.ott_id
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET kingdom_id = kingdom_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS kingdom_id_table ON tmp_bio_ott_taxon_items.bio_kingdom = kingdom_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET phylum_id = phylum_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS phylum_id_table ON tmp_bio_ott_taxon_items.bio_phylum = phylum_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET class_id = class_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS class_id_table ON tmp_bio_ott_taxon_items.bio_class = class_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET order_id = order_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS order_id_table ON tmp_bio_ott_taxon_items.bio_order = order_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET family_id = family_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS family_id_table ON tmp_bio_ott_taxon_items.bio_family = family_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET genus_id = genus_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS genus_id_table ON tmp_bio_ott_taxon_items.bio_genus = genus_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

UPDATE bio_ott_taxon_items
SET domain_id = domain_id_table.id
FROM tmp_bio_ott_taxon_items
JOIN bio_ott_taxon_items AS domain_id_table ON tmp_bio_ott_taxon_items.bio_domain = domain_id_table.name
WHERE bio_ott_taxon_items.ott_id = tmp_bio_ott_taxon_items.uid;

DROP TABLE tmp_bio_ott_taxon_items;

