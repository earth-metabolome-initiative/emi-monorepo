-- Down version of project_states_name_description_trgm_idx
DROP INDEX IF EXISTS project_states_name_description_trgm_idx;
DROP FUNCTION f_concat_project_states_name_description(text, text);