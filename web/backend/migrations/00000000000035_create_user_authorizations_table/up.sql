-- SQL to define the user_authorizations table.
CREATE TABLE user_authorizations (
    id UUID PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    editable_id UUID NOT NULL REFERENCES editables (id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles (id) ON DELETE SET NULL,
    UNIQUE (user_id, editable_id, role_id)
);
