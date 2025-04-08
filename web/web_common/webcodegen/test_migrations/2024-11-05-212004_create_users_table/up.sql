-- SQL query creating a mockup of the users table.
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL CHECK (must_not_be_empty(username)),
    email CITEXT NOT NULL CHECK (must_be_mail(email)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (username),
    UNIQUE (email),
    UNIQUE (username, email)
);

CREATE OR REPLACE FUNCTION js_jpeg(bytea_data BYTEA) RETURNS BOOLEAN AS $$
BEGIN
    -- Check for JPEG (basic check for start of JPEG file)
    RETURN substring(bytea_data for 3) = '\xffd8ff';
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

DROP DOMAIN IF EXISTS jpeg_in CASCADE;
CREATE DOMAIN jpeg_in AS BYTEA
    CONSTRAINT jpeg_in_check CHECK (js_jpeg(VALUE));

DROP TYPE IF EXISTS jpeg CASCADE;
CREATE TYPE jpeg AS (
    value jpeg_in
);

CREATE TYPE norse_gods AS ENUM ('THOR', 'ODIN', 'LOKI');

CREATE TYPE Point2d AS (x DOUBLE PRECISION,y DOUBLE PRECISION);

CREATE TABLE composite_users (
    primary_id INT,
    secondary_id INT,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    user_image jpeg NOT NULL,
    position Point2d NOT NULL,
    god norse_gods NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (primary_id, secondary_id),
    FOREIGN KEY (primary_id) REFERENCES users(id),
    FOREIGN KEY (secondary_id) REFERENCES users(id)
);