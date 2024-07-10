-- SQL defining the login providers table creation.
--
-- The Oauth2 login providers are companies that allow users to login to the
-- application using their credentials. The table is used to store the
-- information about the login providers.
CREATE TABLE IF NOT EXISTS login_providers (
    id INTEGER PRIMARY KEY,
    name text UNIQUE NOT NULL,
    icon_id INTEGER NOT NULL UNIQUE,
    color_id INTEGER NOT NULL UNIQUE,
    client_id_var_name text NOT NULL,
    redirect_uri_var_name text NOT NULL,
    oauth_url text NOT NULL,
    scope text NOT NULL,
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id)
);