-- Down version of sample_states_name_description_trgm_idx
DROP INDEX IF EXISTS sample_states_name_description_trgm_idx;
DROP FUNCTION concat_sample_states_name_description(text, text);