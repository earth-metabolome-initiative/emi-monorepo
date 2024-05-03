CREATE TEMPORARY TABLE tmp_login_providers(
    name TEXT,
    font_awesome_icon TEXT,
    color TEXT,
    client_id_var_name TEXT,
    redirect_uri_var_name TEXT,
    oauth_url TEXT,
    scope TEXT
);

COPY tmp_login_providers
FROM
    '/app/login_providers.csv' DELIMITER ',' CSV HEADER;

INSERT INTO
    login_providers(
        name,
        font_awesome_icon_id,
        color_id,
        client_id_var_name,
        redirect_uri_var_name,
        oauth_url,
        scope
    )
SELECT
    tmp_login_providers.name,
    font_awesome_icons.id,
    colors.id,
    tmp_login_providers.client_id_var_name,
    tmp_login_providers.redirect_uri_var_name,
    tmp_login_providers.oauth_url,
    tmp_login_providers.scope
FROM
    tmp_login_providers
    JOIN font_awesome_icons ON tmp_login_providers.font_awesome_icon = font_awesome_icons.name
    JOIN colors ON tmp_login_providers.color = colors.name;

DROP TABLE tmp_login_providers;