-- We remove the index created in the up.sql file.

DROP INDEX teams_name_description_trgm_idx;
DROP FUNCTION IF EXISTS f_concat_teams_name_description(name text, description text);