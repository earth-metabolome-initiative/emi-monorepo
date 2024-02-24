-- Your SQL goes here
CREATE TABLE organization_user_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a organization user role is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a organization user role is deleted.
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
    ON organization_user_roles FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We insert some of the organization user roles into the organization_user_roles table,
-- such as the organization admin.
DO $$
DECLARE
    editables_id BIGINT;
BEGIN
    -- The root user has ID 1
    INSERT INTO editables (created_by) VALUES (1) RETURNING id INTO editables_id;
    INSERT INTO describables (id, name, description) VALUES (editables_id, 'admin', 'The organization admin has full access to the organization.');
    INSERT INTO organization_user_roles (id) VALUES (editables_id);
END;
$$;