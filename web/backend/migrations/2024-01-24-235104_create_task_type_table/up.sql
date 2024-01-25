-- Your SQL goes here
CREATE TABLE task_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(512) NOT NULL
);