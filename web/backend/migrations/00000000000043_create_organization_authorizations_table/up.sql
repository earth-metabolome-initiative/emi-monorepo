-- SQL defining the organization_authorizations table.
CREATE TABLE organization_authorizations (
    id UUID PRIMARY KEY REFERENCES editables (id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    editable_id UUID NOT NULL REFERENCES editables (id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    UNIQUE (organization_id, editable_id, role_id)
);
