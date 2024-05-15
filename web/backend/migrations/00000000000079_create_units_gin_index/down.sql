-- Drop index on the units table.
-- The index was used to run approximate search queries on the table.
DROP INDEX units_name_description_symbol_trgm_idx;

DROP FUNCTION f_concat_units_name_description_symbol(name text, description text, symbol text);