-- SQL defining the laboratory_users table.
-- The laboratory_users table is a link table that links a user to a laboratory with a role.
-- The user_id, laboratory_id and role columns are used to store the user, laboratory and role, and
-- are used as primary keys. The created_at column is used to store the creation time of the record.
-- Since only a laboratory administrator can add a user to a laboratory, the laboratory_users table
-- also contains a column to specify which administrator added the user to the laboratory.
CREATE TABLE laboratory_users (
    user_id int NOT NULL,
    laboratory_id int NOT NULL,
    role int NOT NULL,
    added_by int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, laboratory_id, role),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (laboratory_id) REFERENCES laboratories (id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users (id) ON DELETE CASCADE
);