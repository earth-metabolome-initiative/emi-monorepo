-- SQL defining the login providers table creation.
--
-- The Oauth2 login providers are companies that allow users to login to the
-- application using their credentials. The table is used to store the
-- information about the login providers.

CREATE TABLE login_providers (
    id SMALLSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO login_providers (name) VALUES ('GitHub');