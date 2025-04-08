-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS teams;

DROP INDEX teams_name_description_trgm_idx;
DROP FUNCTION IF EXISTS concat_teams_name_description(name text, description text);