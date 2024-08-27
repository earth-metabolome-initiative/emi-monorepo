-- Create the teams_users_role_invitations table.
CREATE TABLE IF NOT EXISTS teams_users_role_invitations (
    table_id integer NOT NULL ,
    user_id INTEGER NOT NULL ,
    role_id INTEGER NOT NULL ,
    created_by INTEGER NOT NULL ,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (table_id, user_id),
    FOREIGN KEY (table_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);
