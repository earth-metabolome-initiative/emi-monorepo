CREATE TABLE IF NOT EXISTS icons (
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE icons_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL
);

COPY icons_temp FROM '/app/csvs/icons.csv' DELIMITER ',' CSV HEADER;

INSERT INTO icons (
    name,
    description
) SELECT
    icons_temp.name,
    icons_temp.description
FROM
    icons_temp;

DROP TABLE icons_temp;

CREATE TABLE IF NOT EXISTS colors (
    name TEXT UNIQUE NOT NULL,
    hexadecimal_value TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE colors_temp (
    name TEXT UNIQUE NOT NULL,
    hexadecimal_value TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL
);

COPY colors_temp FROM '/app/csvs/colors.csv' DELIMITER ',' CSV HEADER;

INSERT INTO colors (
    name,
    hexadecimal_value,
    description
) SELECT
    colors_temp.name,
    colors_temp.hexadecimal_value,
    colors_temp.description
FROM
    colors_temp;

DROP TABLE colors_temp;

CREATE TABLE IF NOT EXISTS units (
    name TEXT UNIQUE NOT NULL,
    unit TEXT UNIQUE NOT NULL,
    icon_id SMALLINT NOT NULL REFERENCES icons(id),
    color_id SMALLINT NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE units_temp (
    name TEXT UNIQUE NOT NULL,
    unit TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY units_temp FROM '/app/csvs/units.csv' DELIMITER ',' CSV HEADER;

INSERT INTO units (
    name,
    unit,
    icon_id,
    color_id
) SELECT
    units_temp.name,
    units_temp.unit,
    icons.id,
    colors.id
FROM
    units_temp
    JOIN icons ON units_temp.icons_name = icons.name
    JOIN colors ON units_temp.colors_name = colors.name;

DROP TABLE units_temp;

CREATE TABLE IF NOT EXISTS ranks (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE ranks_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL
);

COPY ranks_temp FROM '/app/csvs/ranks.csv' DELIMITER ',' CSV HEADER;

INSERT INTO ranks (
    name,
    description
) SELECT
    ranks_temp.name,
    ranks_temp.description
FROM
    ranks_temp;

DROP TABLE ranks_temp;

CREATE TABLE IF NOT EXISTS materials (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE materials_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY materials_temp FROM '/app/csvs/materials.csv' DELIMITER ',' CSV HEADER;

INSERT INTO materials (
    name,
    description,
    icon_id,
    color_id
) SELECT
    materials_temp.name,
    materials_temp.description,
    icons.id,
    colors.id
FROM
    materials_temp
    JOIN icons ON materials_temp.icons_name = icons.name
    JOIN colors ON materials_temp.colors_name = colors.name;

DROP TABLE materials_temp;

CREATE TABLE IF NOT EXISTS team_states (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE team_states_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY team_states_temp FROM '/app/csvs/team_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO team_states (
    name,
    description,
    icon_id,
    color_id
) SELECT
    team_states_temp.name,
    team_states_temp.description,
    icons.id,
    colors.id
FROM
    team_states_temp
    JOIN icons ON team_states_temp.icons_name = icons.name
    JOIN colors ON team_states_temp.colors_name = colors.name;

DROP TABLE team_states_temp;

CREATE TABLE IF NOT EXISTS taxa (
    id INTEGER PRIMARY KEY UNIQUE NOT NULL,
    name TEXT NOT NULL,
    parent_id INTEGER,
    rank_id SMALLINT NOT NULL REFERENCES ranks(id)
);
CREATE TEMPORARY TABLE taxa_temp (
    id INTEGER UNIQUE NOT NULL,
    name TEXT NOT NULL,
    parent_id INTEGER,
    ranks_name TEXT
);

COPY taxa_temp FROM '/app/csvs/taxa.csv' DELIMITER ',' CSV HEADER;

INSERT INTO taxa (
    id,
    name,
    parent_id,
    rank_id
) SELECT
    taxa_temp.id,
    taxa_temp.name,
    taxa_temp.parent_id,
    ranks.id
FROM
    taxa_temp
    JOIN ranks ON taxa_temp.ranks_name = ranks.name;

DROP TABLE taxa_temp;

CREATE TABLE IF NOT EXISTS sample_states (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE sample_states_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY sample_states_temp FROM '/app/csvs/sample_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO sample_states (
    name,
    description,
    icon_id,
    color_id
) SELECT
    sample_states_temp.name,
    sample_states_temp.description,
    icons.id,
    colors.id
FROM
    sample_states_temp
    JOIN icons ON sample_states_temp.icons_name = icons.name
    JOIN colors ON sample_states_temp.colors_name = colors.name;

DROP TABLE sample_states_temp;

CREATE TABLE IF NOT EXISTS sample_container_categories (
    name TEXT NOT NULL,
    volume TEXT NOT NULL,
    unit_id SMALLINT NOT NULL REFERENCES units(id),
    material_id SMALLINT NOT NULL REFERENCES materials(id),
    description TEXT NOT NULL,
    icon_id SMALLINT NOT NULL REFERENCES icons(id),
    color_id SMALLINT NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE sample_container_categories_temp (
    name TEXT NOT NULL,
    volume TEXT NOT NULL,
    units_unit VARCHAR(2),
    materials_name TEXT,
    description TEXT NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY sample_container_categories_temp FROM '/app/csvs/sample_container_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO sample_container_categories (
    name,
    volume,
    unit_id,
    material_id,
    description,
    icon_id,
    color_id
) SELECT
    sample_container_categories_temp.name,
    sample_container_categories_temp.volume,
    units.id,
    materials.id,
    sample_container_categories_temp.description,
    icons.id,
    colors.id
FROM
    sample_container_categories_temp
    JOIN units ON sample_container_categories_temp.units_unit = units.unit
    JOIN materials ON sample_container_categories_temp.materials_name = materials.name
    JOIN icons ON sample_container_categories_temp.icons_name = icons.name
    JOIN colors ON sample_container_categories_temp.colors_name = colors.name;

DROP TABLE sample_container_categories_temp;

CREATE TABLE IF NOT EXISTS roles (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE roles_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY roles_temp FROM '/app/csvs/roles.csv' DELIMITER ',' CSV HEADER;

INSERT INTO roles (
    name,
    description,
    icon_id,
    color_id
) SELECT
    roles_temp.name,
    roles_temp.description,
    icons.id,
    colors.id
FROM
    roles_temp
    JOIN icons ON roles_temp.icons_name = icons.name
    JOIN colors ON roles_temp.colors_name = colors.name;

DROP TABLE roles_temp;

CREATE TABLE IF NOT EXISTS project_states (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE project_states_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY project_states_temp FROM '/app/csvs/project_states.csv' DELIMITER ',' CSV HEADER;

INSERT INTO project_states (
    name,
    description,
    icon_id,
    color_id
) SELECT
    project_states_temp.name,
    project_states_temp.description,
    icons.id,
    colors.id
FROM
    project_states_temp
    JOIN icons ON project_states_temp.icons_name = icons.name
    JOIN colors ON project_states_temp.colors_name = colors.name;

DROP TABLE project_states_temp;

CREATE TABLE IF NOT EXISTS permanence_categories (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE permanence_categories_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icons_name TEXT,
    colors_name TEXT
);

COPY permanence_categories_temp FROM '/app/csvs/permanence_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO permanence_categories (
    name,
    description,
    icon_id,
    color_id
) SELECT
    permanence_categories_temp.name,
    permanence_categories_temp.description,
    icons.id,
    colors.id
FROM
    permanence_categories_temp
    JOIN icons ON permanence_categories_temp.icons_name = icons.name
    JOIN colors ON permanence_categories_temp.colors_name = colors.name;

DROP TABLE permanence_categories_temp;

CREATE TABLE IF NOT EXISTS organizations (
    name TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    country TEXT NOT NULL,
    alpha_two_code VARCHAR(2) NOT NULL,
    state_province TEXT,
    domain TEXT UNIQUE NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE organizations_temp (
    name TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    country TEXT NOT NULL,
    alpha_two_code VARCHAR(2) NOT NULL,
    state_province TEXT,
    domain TEXT UNIQUE NOT NULL
);

COPY organizations_temp FROM PROGRAM 'gzip -dc /app/csvs/organizations.csv.gz' DELIMITER ',' CSV HEADER;

INSERT INTO organizations (
    name,
    url,
    country,
    alpha_two_code,
    state_province,
    domain
) SELECT
    organizations_temp.name,
    organizations_temp.url,
    organizations_temp.country,
    organizations_temp.alpha_two_code,
    organizations_temp.state_province,
    organizations_temp.domain
FROM
    organizations_temp;

DROP TABLE organizations_temp;

CREATE TABLE IF NOT EXISTS observation_subjects (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon TEXT UNIQUE NOT NULL,
    color TEXT NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE observation_subjects_temp (
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    icon TEXT UNIQUE NOT NULL,
    color TEXT NOT NULL
);

COPY observation_subjects_temp FROM '/app/csvs/observation_subjects.csv' DELIMITER ',' CSV HEADER;

INSERT INTO observation_subjects (
    name,
    description,
    icon,
    color
) SELECT
    observation_subjects_temp.name,
    observation_subjects_temp.description,
    observation_subjects_temp.icon,
    observation_subjects_temp.color
FROM
    observation_subjects_temp;

DROP TABLE observation_subjects_temp;

CREATE TABLE IF NOT EXISTS nameplate_categories (
    name VARCHAR(9) UNIQUE NOT NULL,
    material_id SMALLINT UNIQUE NOT NULL REFERENCES materials(id),
    permanence VARCHAR(9) UNIQUE NOT NULL,
    description VARCHAR(162) UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE nameplate_categories_temp (
    name VARCHAR(9) UNIQUE NOT NULL,
    materials_name VARCHAR(8),
    permanence VARCHAR(9) UNIQUE NOT NULL,
    description VARCHAR(162) UNIQUE NOT NULL,
    icons_name VARCHAR(3),
    colors_name VARCHAR(4)
);

COPY nameplate_categories_temp FROM '/app/csvs/nameplate_categories.csv' DELIMITER ',' CSV HEADER;

INSERT INTO nameplate_categories (
    name,
    material_id,
    permanence,
    description,
    icon_id,
    color_id
) SELECT
    nameplate_categories_temp.name,
    materials.id,
    nameplate_categories_temp.permanence,
    nameplate_categories_temp.description,
    icons.id,
    colors.id
FROM
    nameplate_categories_temp
    JOIN materials ON nameplate_categories_temp.materials_name = materials.name
    JOIN icons ON nameplate_categories_temp.icons_name = icons.name
    JOIN colors ON nameplate_categories_temp.colors_name = colors.name;

DROP TABLE nameplate_categories_temp;

CREATE TABLE IF NOT EXISTS login_providers (
    name VARCHAR(6) UNIQUE NOT NULL,
    icon_id SMALLINT UNIQUE NOT NULL REFERENCES icons(id),
    color_id SMALLINT UNIQUE NOT NULL REFERENCES colors(id),
    client_id_var_name VARCHAR(16) UNIQUE NOT NULL,
    redirect_uri_var_name VARCHAR(19) UNIQUE NOT NULL,
    oauth_url VARCHAR(40) UNIQUE NOT NULL,
    scope VARCHAR(10) UNIQUE NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE login_providers_temp (
    name VARCHAR(6) UNIQUE NOT NULL,
    icons_name VARCHAR(6),
    colors_name VARCHAR(5),
    client_id_var_name VARCHAR(16) UNIQUE NOT NULL,
    redirect_uri_var_name VARCHAR(19) UNIQUE NOT NULL,
    oauth_url VARCHAR(40) UNIQUE NOT NULL,
    scope VARCHAR(10) UNIQUE NOT NULL
);

COPY login_providers_temp FROM '/app/csvs/login_providers.csv' DELIMITER ',' CSV HEADER;

INSERT INTO login_providers (
    name,
    icon_id,
    color_id,
    client_id_var_name,
    redirect_uri_var_name,
    oauth_url,
    scope
) SELECT
    login_providers_temp.name,
    icons.id,
    colors.id,
    login_providers_temp.client_id_var_name,
    login_providers_temp.redirect_uri_var_name,
    login_providers_temp.oauth_url,
    login_providers_temp.scope
FROM
    login_providers_temp
    JOIN icons ON login_providers_temp.icons_name = icons.name
    JOIN colors ON login_providers_temp.colors_name = colors.name;

DROP TABLE login_providers_temp;

CREATE TABLE IF NOT EXISTS document_formats (
    extension TEXT UNIQUE NOT NULL,
    mime_type TEXT NOT NULL,
    description TEXT NOT NULL,
    icon_id SMALLINT NOT NULL REFERENCES icons(id),
    color TEXT NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE document_formats_temp (
    extension TEXT UNIQUE NOT NULL,
    mime_type TEXT NOT NULL,
    description TEXT NOT NULL,
    icons_name TEXT,
    color TEXT NOT NULL
);

COPY document_formats_temp FROM '/app/csvs/document_formats.csv' DELIMITER ',' CSV HEADER;

INSERT INTO document_formats (
    extension,
    mime_type,
    description,
    icon_id,
    color
) SELECT
    document_formats_temp.extension,
    document_formats_temp.mime_type,
    document_formats_temp.description,
    icons.id,
    document_formats_temp.color
FROM
    document_formats_temp
    JOIN icons ON document_formats_temp.icons_name = icons.name;

DROP TABLE document_formats_temp;

CREATE TABLE IF NOT EXISTS countries (
    ISO VARCHAR(2) UNIQUE NOT NULL,
    emoji VARCHAR(8) UNIQUE NOT NULL,
    unicode VARCHAR(15) UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL,
    id SMALLSERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE countries_temp (
    ISO VARCHAR(2) UNIQUE NOT NULL,
    emoji VARCHAR(8) UNIQUE NOT NULL,
    unicode VARCHAR(15) UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
);

COPY countries_temp FROM '/app/csvs/countries.csv' DELIMITER ',' CSV HEADER;

INSERT INTO countries (
    ISO,
    emoji,
    unicode,
    name
) SELECT
    countries_temp.ISO,
    countries_temp.emoji,
    countries_temp.unicode,
    countries_temp.name
FROM
    countries_temp;

DROP TABLE countries_temp;

