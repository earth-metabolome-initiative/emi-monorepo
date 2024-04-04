-- Set the default index of the projects column to gen_random_uuid()

ALTER TABLE projects ALTER COLUMN id SET DEFAULT gen_random_uuid();