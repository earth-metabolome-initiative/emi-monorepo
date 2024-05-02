-- Drop index on the item_categories table.
-- The index was used to run approximate search queries on the table.
DROP INDEX item_categories_name_description_trgm_idx;

DROP FUNCTION f_concat_item_categories_name_description(name text, description text);