-- SQL defining the organizations table.
-- An organization represents a continuous-world organization such as a University, or a company.
-- Users, projects and teams can be associated to one or more organizations.
-- An organization has a unique name, description, and may have a parent organization.
-- Furthermore, an organization may have one or more child organizations.
-- The created_at and updated_at columns are used to store the creation and last update time of the record.
-- An organization may have a logo, and a URL for the organization website.
-- The organization abstraction, as well as the team abstraction, are primarily used to manage access to projects,
-- so to avoid having to manage access to each user individually.
CREATE TABLE organizations (
  id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE,
  state_id UUID DEFAULT NULL REFERENCES organization_states(id) ON DELETE SET NULL,
  parent_organization_id UUID DEFAULT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  logo_id UUID DEFAULT NULL REFERENCES documents(id) ON DELETE SET NULL,
  website_url VARCHAR(255) DEFAULT NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a organization is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a organization is deleted.
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
    ON organizations FOR EACH ROW EXECUTE FUNCTION delete_editables();
    