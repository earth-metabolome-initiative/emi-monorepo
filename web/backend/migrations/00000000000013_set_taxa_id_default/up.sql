-- Set the default index of the taxa column to gen_random_uuid()

ALTER TABLE taxa ALTER COLUMN id SET DEFAULT gen_random_uuid();