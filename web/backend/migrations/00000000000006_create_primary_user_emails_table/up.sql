-- SQL defining the primary user emails table creation.
CREATE TABLE primary_user_emails (
    id UUID PRIMARY KEY REFERENCES user_emails (id) ON DELETE CASCADE
);