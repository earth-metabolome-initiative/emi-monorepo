-- Drop index on the observation_subjects table.
-- The index was used to run approximate search queries on the table.
DROP INDEX observation_subjects_name_description_trgm_idx;

DROP FUNCTION concat_observation_subjects_name_description(name text, description text);