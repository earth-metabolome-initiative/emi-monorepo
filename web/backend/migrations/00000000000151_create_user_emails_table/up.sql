-- SQL defining the user emails table creation.
-- These are the emails that the users have associated with their accounts
-- and are used to send notifications and other important information.
-- They are all associated to a login provider.

CREATE TABLE IF NOT EXISTS user_emails (
    id INTEGER PRIMARY KEY,
    email text NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    login_provider_id INTEGER NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    primary_email BOOLEAN NOT NULL DEFAULT TRUE,
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
        AND created_by IS DISTINCT FROM NEW.created_by
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

-- SQL trigger making sure that when a user email is marked as primary, all other emails
-- associated with the same user are marked as not primary.
CREATE FUNCTION update_primary_email() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.primary_email THEN
        UPDATE user_emails
        SET primary_email = FALSE
        WHERE created_by = NEW.created_by
        AND id IS DISTINCT FROM NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER primary_email_trigger
BEFORE INSERT OR UPDATE ON user_emails
FOR EACH ROW
EXECUTE FUNCTION update_primary_email();
