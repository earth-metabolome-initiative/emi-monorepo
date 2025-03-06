-- SQL query creating a function to check whether the provided age is greater than 18.
CREATE OR REPLACE FUNCTION check_age(age INT) RETURNS BOOLEAN AS $$
BEGIN
    IF age > 18 THEN
        RETURN TRUE;
    ELSE
        RETURN FALSE;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_young_age(age INT) RETURNS BOOLEAN AS $$
BEGIN
    IF age < 50 THEN
        RETURN TRUE;
    ELSE
        RETURN FALSE;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_strings_different(str1 VARCHAR, str2 VARCHAR) RETURNS BOOLEAN AS $$
BEGIN
    IF str1 <> str2 THEN
        RETURN TRUE;
    ELSE
        RETURN FALSE;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_values_different(int1 INT, int2 INT) RETURNS BOOLEAN AS $$
BEGIN
    IF int1 <> int2 THEN
        RETURN TRUE;
    ELSE
        RETURN FALSE;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_non_empty_string(str VARCHAR) RETURNS BOOLEAN AS $$
BEGIN
    IF str <> '' THEN
        RETURN TRUE;
    ELSE
        RETURN FALSE;
    END IF;
END;
$$ LANGUAGE plpgsql;


-- SQL query creating a mockup of the constrained users table.
CREATE TABLE constrained_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL CHECK (check_non_empty_string(username)),
    email VARCHAR(255) NOT NULL,
    age INT NOT NULL CHECK (check_age(age) AND check_young_age(age)),
    fortune FLOAT NOT NULL CHECK (fortune > 10000000),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (username),
    UNIQUE (email),
    UNIQUE (username, email),
    CHECK (check_strings_different(username, email)),
    CHECK (check_young_age(age) AND check_young_age(age) AND check_age(age))
);


-- SQL query creating a mockup of the constrained samples table.
CREATE TABLE constrained_samples (
    id SERIAL PRIMARY KEY,
    samplename VARCHAR(255) NOT NULL CHECK (check_non_empty_string(samplename)),
    sanitized_samplename VARCHAR(255) NOT NULL CHECK (check_non_empty_string(sanitized_samplename)),
    amount_initial INT NOT NULL CHECK (amount_initial > 10),
    amount_final INT NOT NULL CHECK (amount_final > 2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (samplename),
    UNIQUE (samplename, id),
    CHECK (check_values_different(amount_initial, amount_final)),
    CHECK (check_strings_different(samplename, sanitized_samplename))
);


-- SQL query creating a mockup of the unconstrained samples table.
CREATE TABLE unconstrained_samples (
    id SERIAL PRIMARY KEY,
    samplename VARCHAR(255) NOT NULL CHECK (check_non_empty_string(samplename)),
    sanitized_samplename VARCHAR(255) NOT NULL CHECK (check_non_empty_string(sanitized_samplename)),
    amount_initial INT NOT NULL CHECK (amount_initial > 10),
    amount_final INT NOT NULL CHECK (amount_final > 2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (samplename),
    UNIQUE (samplename, id)
    CHECK (check_strings_different(username, email)),
    CHECK (check_young_age(age) AND check_young_age(age) AND check_age(age))
);