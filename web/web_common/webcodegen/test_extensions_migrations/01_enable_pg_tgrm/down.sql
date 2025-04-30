-- SQL migration to disable the pg_trgm extension
DROP EXTENSION IF EXISTS pg_trgm;
