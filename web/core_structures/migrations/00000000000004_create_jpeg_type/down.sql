-- We revert everything that we did in the up.sql file
-- Drop the type jpeg
DROP TYPE IF EXISTS jpeg;
-- Drop the domain jpeg_in
DROP DOMAIN IF EXISTS jpeg_in;
-- Drop the function js_jpeg
DROP FUNCTION IF EXISTS js_jpeg(bytea);