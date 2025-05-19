CREATE TABLE IF NOT EXISTS user_emails (
    id SERIAL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    created_by INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    primary_email BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS temporary_user_emails (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email text NOT NULL,
    login_provider_id SMALLINT NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    validated BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (email, login_provider_id)
);

CREATE TABLE IF NOT EXISTS email_providers (
    email_id INTEGER NOT NULL REFERENCES user_emails (id) ON DELETE CASCADE,
    login_provider_id SMALLINT NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    PRIMARY KEY (email_id, login_provider_id)
);
