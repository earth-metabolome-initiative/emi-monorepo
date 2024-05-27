-- Down version of team_states_name_description_trgm_idx
DROP INDEX IF EXISTS team_states_name_description_trgm_idx;
DROP FUNCTION IF EXISTS concat_team_states_name_description(name TEXT, description TEXT);