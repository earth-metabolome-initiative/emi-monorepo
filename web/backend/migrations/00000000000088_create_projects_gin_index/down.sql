-- We remove the index created in the up.sql file.

DROP INDEX projects_name_description_trgm_idx;
DROP FUNCTION IF EXISTS concat_projects_name_description(name text, description text);