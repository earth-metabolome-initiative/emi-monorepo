-- Removes default value from the id projects column, which was set
-- to gen_random_uuid during the up.sql migration.
ALTER TABLE projects ALTER COLUMN id DROP DEFAULT;