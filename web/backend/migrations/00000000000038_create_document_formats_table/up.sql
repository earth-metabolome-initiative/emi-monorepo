-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE document_formats (
  id SERIAL PRIMARY KEY,
  extension VARCHAR(255) NOT NULL UNIQUE,
  mime_type VARCHAR(255) NOT NULL
);

-- We insert the default document formats.
-- We start by inserting the editables that indexes the formats.
DO $$
DECLARE
  extensions VARCHAR(255) [ ] := ARRAY [ 'pdf',
  'jpg',
  'png',
  'mgf',
  'jpeg',
  'csv',
  'txt',
  'webp',
  'json' ];

mime_types VARCHAR(255) [ ] := ARRAY [ 'application/pdf',
'image/jpeg',
'image/png',
'text/mgf',
'image/jpeg',
'text/csv',
'text/plain',
'image/webp',
'application/json' ];

BEGIN
  -- Insert the formats in the document_formats table
  INSERT INTO
    document_formats (extension, mime_type)
  SELECT
    unnest(extensions),
    unnest(mime_types);

END;

$$;