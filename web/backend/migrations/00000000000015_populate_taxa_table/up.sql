-- Your SQL goes here
-- Create a temporary table to hold the CSV data
CREATE TEMP TABLE temp_taxon_csv_data (
    taxon_id INTEGER,
    taxon_name TEXT
);

-- Load the CSV data into the temporary table
COPY temp_taxon_csv_data(taxon_id, taxon_name)
FROM '/app/taxons.csv'
WITH (FORMAT CSV, HEADER true, DELIMITER E'\t');

-- We insert in the taxa table the values from the temporary table
INSERT INTO taxa (name, ncbi_taxon_id)
SELECT taxon_name, taxon_id
FROM temp_taxon_csv_data;

-- Drop the temporary table
DROP TABLE temp_taxon_csv_data;