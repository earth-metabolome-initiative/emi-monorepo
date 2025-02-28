-- This file should undo anything in `up.sql`
DROP TABLE users;
DROP FUNCTION check_age(INT);
DROP FUNCTION check_young_age(INT);
DROP FUNCTION check_strings_different(VARCHAR, VARCHAR);
DROP FUNCTION check_non_empty_string(VARCHAR);
