CREATE TABLE IF NOT EXISTS user_emails (
    id SERIAL PRIMARY KEY,
    email text NOT NULL UNIQUE CHECK (must_be_email(email)),
    created_by INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    primary_email BOOLEAN NOT NULL DEFAULT TRUE,
    -- There can only be one user associated with an email address
    UNIQUE (email, created_by),
    -- There can only be one primary email address per user
    UNIQUE (created_by, primary_email)
);

CREATE TABLE IF NOT EXISTS email_providers (
    email_id INTEGER NOT NULL REFERENCES user_emails (id) ON DELETE CASCADE,
    login_provider_id SMALLINT NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    PRIMARY KEY (email_id, login_provider_id)
);
