-- Drop the index created in the up migration
DROP INDEX IF EXISTS users_name_trgm_idx;
DROP FUNCTION f_concat_users_name(text);