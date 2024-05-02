-- This is a no-op SQL statement
INSERT INTO
    login_providers (name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope)
VALUES
    ('GitHub', 'fa-github', 'GITHUB_CLIENT_ID', 'GITHUB_REDIRECT_URI', 'https://github.com/login/oauth/authorize', 'user:email');