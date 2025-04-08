-- SQL needed to drop the projects table.

DROP TABLE IF EXISTS projects;

DROP INDEX projects_name_description_trgm_idx;
DROP FUNCTION IF EXISTS concat_projects_name_description(name text, description text);