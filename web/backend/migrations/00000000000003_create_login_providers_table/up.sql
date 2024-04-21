-- SQL defining the login providers table creation.
--
-- The Oauth2 login providers are companies that allow users to login to the
-- application using their credentials. The table is used to store the
-- information about the login providers.
CREATE TABLE login_providers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    font_awesome_icon VARCHAR(255) NOT NULL,
    client_id_var_name VARCHAR(255) NOT NULL,
    redirect_uri_var_name VARCHAR(255) NOT NULL,
    oauth_url VARCHAR(255) NOT NULL,
    scope VARCHAR(255) NOT NULL
);

INSERT INTO
    login_providers (name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope)
VALUES
    ('GitHub', 'fa-github', 'GITHUB_CLIENT_ID', 'GITHUB_REDIRECT_URI', 'https://github.com/login/oauth/authorize', 'user:email');