-- SQL defining the user emails table creation.
-- These are the emails that the users have associated with their accounts
-- and are used to send notifications and other important information.
-- They are all associated to a login provider.

CREATE TABLE user_emails (
    email VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    login_provider_id INTEGER NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, login_provider_id),
    UNIQUE (email, login_provider_id)
);