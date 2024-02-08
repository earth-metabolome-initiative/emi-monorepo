-- SQL defining the organizations table.
-- An organization represents a real-world organization such as a University, or a company.
-- Users, projects and teams can be associated to one or more organizations.
-- An organization has a unique name, description, and may have a parent organization.
-- Furthermore, an organization may have one or more child organizations.
-- The created_at and updated_at columns are used to store the creation and last update time of the record.
-- An organization may have a logo, and a URL for the organization website.
-- The organization abstraction, as well as the team abstraction, are primarily used to manage access to projects,
-- so to avoid having to manage access to each user individually.
CREATE TABLE organizations (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  state INT NOT NULL DEFAULT 0,
  parent_organization_id INT DEFAULT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  logo_path VARCHAR(255) DEFAULT NULL,
  website_url VARCHAR(255) DEFAULT NULL,
  FOREIGN KEY (parent_organization_id) REFERENCES organizations (id) ON DELETE CASCADE
);
