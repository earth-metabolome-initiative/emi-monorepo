-- This is a no-op SQL statement

CREATE TEMPORARY TABLE tmp_organizations(
    name TEXT,
    url TEXT,
    country TEXT,
    alpha_two_code TEXT,
    state_province TEXT,
    domain TEXT
);

COPY tmp_organizations
FROM PROGRAM 'gzip -dc /app/organizations.csv.gz' DELIMITER ',' CSV HEADER;

INSERT INTO
    organizations(
        name,
        url,
        country_id,
        domain
    )
SELECT 
    tmp_organizations.name,
    tmp_organizations.url,
    countries.id,
    tmp_organizations.domain
FROM
    tmp_organizations
    JOIN countries ON tmp_organizations.alpha_two_code = countries.ISO;

DO $$ DECLARE tmp_organizations_count INTEGER;

organizations_count INTEGER;

BEGIN
SELECT
    COUNT(*) INTO tmp_organizations_count
FROM
    tmp_organizations;

SELECT
    COUNT(*) INTO organizations_count
FROM
    organizations;

IF tmp_organizations_count <> organizations_count THEN RAISE EXCEPTION 'The number of rows in the temporary table is different from the number of rows in the table';
    
END IF;

END $$;

DROP TABLE tmp_organizations;