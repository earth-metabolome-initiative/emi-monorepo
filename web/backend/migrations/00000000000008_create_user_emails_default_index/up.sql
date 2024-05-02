-- This up migration replaces the INTEGER primary key with a SERIAL primary key.
-- This migration is intended to be used in the backend.
-- This migration is automatically generated.

CREATE SEQUENCE user_emails_id_seq;
ALTER TABLE user_emails ALTER COLUMN id SET DEFAULT nextval('user_emails_id_seq');
ALTER TABLE user_emails ALTER COLUMN id SET NOT NULL;
ALTER SEQUENCE user_emails_id_seq OWNED BY user_emails.id;
