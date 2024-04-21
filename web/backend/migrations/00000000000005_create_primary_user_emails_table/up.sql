-- SQL defining the primary user emails table creation.
CREATE TABLE primary_user_emails (
    id INTEGER PRIMARY KEY REFERENCES user_emails (id) ON DELETE CASCADE
);