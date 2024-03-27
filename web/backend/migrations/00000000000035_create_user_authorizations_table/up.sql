-- SQL to define the user_authorizations table.
CREATE TABLE user_authorizations (
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    editable_id UUID NOT NULL REFERENCES editables (id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles (id) ON DELETE SET NULL,
    PRIMARY KEY (user_id, editable_id, role_id)
);
