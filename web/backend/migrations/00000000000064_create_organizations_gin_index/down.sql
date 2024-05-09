-- Drop index on the organizations table.
-- The index was used to run approximate search queries on the table.

DROP INDEX organizations_name_trgm_idx;
