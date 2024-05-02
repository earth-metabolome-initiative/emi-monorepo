-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS notifications;
DROP TRIGGER IF EXISTS notify_user_trigger ON notifications;
-- DROP FUNCTION IF EXISTS notify_user();

