-- This file should undo anything in `up.sql`
DELETE FROM users WHERE first_name = 'root' AND last_name = 'user';