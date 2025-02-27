-- SQL query creating a mockup of the constrained users table.
CREATE TABLE constrained_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL CHECK (username <> ''),
    email VARCHAR(255) NOT NULL,
    age INT NOT NULL CHECK (age > 18),
    fortune FLOAT NOT NULL CHECK (fortune > 10000000),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (username),
    UNIQUE (email),
    UNIQUE (username, email),
    CHECK (username <> email)
);