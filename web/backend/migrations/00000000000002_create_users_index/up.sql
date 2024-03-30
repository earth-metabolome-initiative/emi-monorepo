-- Create an index to search approximately the composite columns of
-- the users table, including first_name, middle_name, and last_name.

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX users_name_trgm_idx ON users USING gin (first_name gin_trgm_ops, middle_name gin_trgm_ops, last_name gin_trgm_ops);
