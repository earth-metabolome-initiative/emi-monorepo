-- Create the spectra_collections_users_roles table.
CREATE TABLE IF NOT EXISTS spectra_collections_users_roles (
    table_id integer NOT NULL REFERENCES spectra_collections(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    role_id INTEGER NOT NULL REFERENCES roles(id),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (table_id, user_id, role_id)
);
