-- Loads from the file taxa.csv the table taxa
--

-- Since we are loading the table from a CSV and there are constraints between
-- child and parent rows (among others) and the CSV is not sorted in any way,
-- we need to disable the constraints before loading the data and re-enable them
-- after the data is loaded.

ALTER TABLE taxa DISABLE TRIGGER ALL;

COPY taxa(
    ott_id,
    name,
    ncbi_id,
    gbif_id,
    irmng_id,
    worms_id,
    parent_id,
    kingdom_id,
    phylum_id,
    class_id,
    order_id,
    family_id,
    genus_id,
    domain_id,
    wikidata_id,
    icon_id,
    color_id,
    ott_rank_id
)
FROM PROGRAM 'gzip -dc /app/taxa.csv.gz'  DELIMITER ',' CSV HEADER;

ALTER TABLE taxa ENABLE TRIGGER ALL;