-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE document_formats (
  id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  description TEXT
);