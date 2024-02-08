-- SQL defining the laboratories table.
-- A laboratory is a place where experiments are conducted. The laboratory table
-- contains information about the laboratory, such as the name, description, and
-- the organization that the laboratory belongs to. The created_at and updated_at
-- columns are used to store the creation and last update time of the record.
CREATE TABLE laboratories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  organization_id INT NOT NULL,
  address VARCHAR(255) DEFAULT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (organization_id) REFERENCES organizations (id) ON DELETE CASCADE
);