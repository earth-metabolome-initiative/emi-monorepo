-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE IF NOT EXISTS document_formats (
  id INTEGER PRIMARY KEY,
  extension VARCHAR(255) NOT NULL UNIQUE,
  mime_type VARCHAR(255) NOT NULL
);
