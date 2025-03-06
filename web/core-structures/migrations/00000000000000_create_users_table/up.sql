-- We need to create a table to store users
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    picture BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- We need to create an index on the name column to speed up queries
CREATE INDEX IF NOT EXISTS users_name_trgm_idx ON users USING gin (
  name gin_trgm_ops
);

-- Since users may have multiple organizations, we need a join table to represent this relationship
CREATE TABLE IF NOT EXISTS user_organizations (
    user_id INTEGER NOT NULL,
    organization_id SMALLINT NOT NULL,
    PRIMARY KEY (user_id, organization_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id) REFERENCES organizations (id) ON DELETE CASCADE
);