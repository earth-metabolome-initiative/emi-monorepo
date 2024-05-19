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
        icon_id,
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

-- now we want to assert that the number of lines in the sample_states table is the same as the number 
-- of lines in the tmp_sample_states table
DO $$ DECLARE tmp_login_providers_count INTEGER;

login_providers_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_login_providers_count
FROM
    tmp_login_providers;

SELECT
    COUNT(*) INTO login_providers_count
FROM
    login_providers;

IF tmp_login_providers_count <> login_providers_count THEN RAISE EXCEPTION 'The number of rows in the tmp_login_providers table is different from the number of rows in the login_providers table';

END IF;

END $$;

DROP TABLE tmp_login_providers;