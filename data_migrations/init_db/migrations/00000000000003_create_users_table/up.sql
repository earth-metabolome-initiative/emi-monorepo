CREATE TABLE IF NOT EXISTS temporary_user (
    id SERIAL PRIMARY KEY,
    email text NOT NULL CHECK (must_be_email(email)),
    login_provider_id SMALLINT NOT NULL REFERENCES login_providers (id) ON DELETE CASCADE,
    UNIQUE (email, login_provider_id)
);

-- We need to create a table to store users
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL CHECK (must_be_paragraph(first_name)),
    last_name TEXT NOT NULL CHECK (must_be_paragraph(last_name)),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Since users may have multiple organizations, we need a join table to represent this relationship
CREATE TABLE IF NOT EXISTS user_organizations (
    user_id INTEGER NOT NULL,
    organization_id SMALLINT NOT NULL,
    PRIMARY KEY (user_id, organization_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id) REFERENCES organizations (id) ON DELETE CASCADE
);