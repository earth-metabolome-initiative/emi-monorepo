-- Drop the index created in the up migration
DROP INDEX IF EXISTS roles_name_trgm_idx;
DROP FUNCTION concat_roles_name(text, text);