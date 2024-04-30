-- SQL defining the user emails table creation.
-- These are the emails that the users have associated with their accounts
-- and are used to send notifications and other important information.
-- They are all associated to a login provider.

CREATE TABLE IF NOT EXISTS user_emails (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    login_provider_id INTEGER NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    CONSTRAINT unique_email_provider UNIQUE (email, login_provider_id)
);

-- SQL defining the check_email_unique function creation.
-- This function is used to check if the email is unique per user, meaning that
-- two distinct users may not have the same email, even with different login providers.
CREATE FUNCTION check_email_unique() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1 
        FROM user_emails 
        WHERE email = NEW.email 
        AND user_id IS DISTINCT FROM NEW.user_id
        AND NEW.id IS DISTINCT FROM id
    ) THEN
        RAISE EXCEPTION 'Email already exists for the given login provider';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER email_unique_trigger
BEFORE INSERT OR UPDATE ON user_emails
FOR EACH ROW
EXECUTE FUNCTION check_email_unique();