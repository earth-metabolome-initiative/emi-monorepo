-- This file should undo anything in `up.sql`
DROP TRIGGER users_notify_trigger;
DROP FUNCTION notify_user();
DROP TABLE IF EXISTS notifications;
