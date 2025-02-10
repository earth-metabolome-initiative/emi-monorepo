-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    middle_name TEXT,
    last_name TEXT NOT NULL,
    description TEXT,
    picture BYTEA NOT NULL,
    organization_id SMALLINT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
);

CREATE OR REPLACE FUNCTION concat_users_name(
  first_name text,
  last_name text
) RETURNS text AS $$
BEGIN
  RETURN first_name || ' ' || last_name;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;


CREATE INDEX IF NOT EXISTS users_name_trgm_idx ON users USING gin (
  concat_users_name(first_name, last_name) gin_trgm_ops
);