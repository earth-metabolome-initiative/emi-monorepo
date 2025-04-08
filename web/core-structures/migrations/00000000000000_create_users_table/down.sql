-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS users;

DROP INDEX IF EXISTS users_name_trgm_idx;
DROP FUNCTION concat_users_name(text, text);