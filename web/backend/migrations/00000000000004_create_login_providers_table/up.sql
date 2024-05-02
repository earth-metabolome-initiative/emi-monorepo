-- SQL defining the login providers table creation.
--
-- The Oauth2 login providers are companies that allow users to login to the
-- application using their credentials. The table is used to store the
-- information about the login providers.
CREATE TABLE IF NOT EXISTS login_providers (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    font_awesome_icon VARCHAR(255) NOT NULL,
    client_id_var_name VARCHAR(255) NOT NULL,
    redirect_uri_var_name VARCHAR(255) NOT NULL,
    oauth_url VARCHAR(255) NOT NULL,
    scope VARCHAR(255) NOT NULL
);
