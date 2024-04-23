-- This file should undo anything in `up.sql`

-- Remove the inserted data from the taxa table
TRUNCATE taxa;

-- Drop the temporary table
DROP TABLE IF EXISTS temp_taxon_csv_data;