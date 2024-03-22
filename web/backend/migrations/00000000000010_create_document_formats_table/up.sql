-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE document_formats (
  id UUID PRIMARY KEY REFERENCES editables(id) ON
  DELETE
    CASCADE REFERENCES describables(id) ON
  DELETE
    CASCADE,
    mime_type VARCHAR(255) NOT NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a document format is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a document format is deleted.
CREATE
OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$ BEGIN
  DELETE FROM
    editables
  WHERE
    id = OLD .id;

RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
  ON document_formats FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We insert the default document formats.
-- We start by inserting the editables that indexes the formats.
DO $$
DECLARE
  root_user_id UUID;

editables_ids UUID [ ];

extensions VARCHAR(255) [ ] := ARRAY [ 'pdf',
'jpg',
'png',
'mgf',
'jpeg',
'csv',
'txt',
'webp',
'json' ];

descriptions VARCHAR(255) [ ] := ARRAY [ 'Portable Document Format',
'Joint Photographic Experts Group',
'Portable Network Graphics',
'Mascot generic format files',
'Joint Photographic Experts Group',
'Comma-separated values',
'Plain text',
'WebP image format',
'JavaScript Object Notation' ];

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
  -- Retrieve the id of the root user
  SELECT
    id INTO root_user_id
  FROM
    users
  WHERE
    first_name = 'root'
    AND last_name = 'user';

-- Insert the editables that index the formats
WITH inserted_rows AS (
  INSERT INTO
    editables (created_by)
  SELECT
    root_user_id
  FROM
    generate_series(1, array_length(extensions, 1)) RETURNING id
)
SELECT
  array_agg(id) INTO editables_ids
FROM
  inserted_rows;

-- Insert the description of the formats in the describables table
INSERT INTO
  describables (id, name, description)
SELECT
  unnest(editables_ids),
  unnest(extensions),
  unnest(descriptions);

-- Insert the formats in the document_formats table
INSERT INTO
  document_formats (id, mime_type)
SELECT
  unnest(editables_ids),
  unnest(mime_types);

END;

$$;