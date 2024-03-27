-- Your SQL goes here
-- We insert the root user, who is the first user of the system.
-- We will assign to root the creation of the records insert during
-- the migration process.
INSERT INTO
    users (first_name, last_name)
VALUES
    ('root', 'user');

