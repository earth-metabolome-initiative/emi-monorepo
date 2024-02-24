-- SQL defining the organization_projects table.
-- This table defines a link from an organization to a project, with a viewer role.
-- All users that are part of the organization will be able to view the project.
-- The organization_id and project_id columns are used to store the organization and project,
-- which are used as primary keys. The created_at column is used to store the creation time of the record.
-- Since only an organization administrator can add link a project to an organization,
-- the organization_projects table also contains a column to specify which administrator
-- added the project to the organization.
CREATE TABLE organization_projects (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    organization_id BIGINT NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    project_id BIGINT NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES team_user_roles (id) ON DELETE CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when an organization project is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when an organization project is deleted.
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
    ON organization_projects FOR EACH ROW EXECUTE FUNCTION delete_editables();
