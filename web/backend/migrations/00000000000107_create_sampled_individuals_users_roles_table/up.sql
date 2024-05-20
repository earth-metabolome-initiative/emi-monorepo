-- Create the sampled_individuals_users_roles table.
CREATE TABLE IF NOT EXISTS sampled_individuals_users_roles (
    table_id uuid NOT NULL REFERENCES sampled_individuals(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (table_id, user_id)
);
