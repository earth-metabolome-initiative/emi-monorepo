-- Removes default value from the id taxa column, which was set
-- to gen_random_uuid during the up.sql migration.
ALTER TABLE taxa ALTER COLUMN id DROP DEFAULT;