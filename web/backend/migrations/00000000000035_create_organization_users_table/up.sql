-- SQL to define the organization_users table.
-- A user appears in the organization_users table if they are a member of an organization.
-- The organization_users table is a many-to-many relation between users and organizations.
-- When either the user or the organization is deleted, the organization_users row should be deleted as well.
-- The role column is used to store the role of the user in the organization.
-- The created_at column is used to store the creation time of the record.
-- Since an administrator needs to add a user to an organization, the organization_users table
-- also contains a column to specify which administrator added the user to the organization.
CREATE TABLE organization_users (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    organization_id BIGINT NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES organization_user_roles (id) ON DELETE SET NULL,
    UNIQUE (user_id, organization_id, role_id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an organization user is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an organization user is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON organization_users FOR EACH ROW EXECUTE FUNCTION delete_editables();