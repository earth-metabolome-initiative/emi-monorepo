-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE notifications_id_seq;
ALTER TABLE notifications ALTER COLUMN id SET DEFAULT nextval('notifications_id_seq');
ALTER TABLE notifications ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE notifications_id_seq OWNED BY notifications.id;
