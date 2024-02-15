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
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
  state_id BIGINT DEFAULT NULL REFERENCES organization_states(id) ON DELETE SET NULL,
  parent_organization_id BIGINT DEFAULT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  logo_id BIGINT DEFAULT NULL REFERENCES documents(id) ON DELETE SET NULL,
  website_url VARCHAR(255) DEFAULT NULL
);
