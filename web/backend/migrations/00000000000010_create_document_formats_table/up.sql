-- SQL defining the document_formats table creation.
-- The taple specifies the set of possible document formats that can be used in the system.
CREATE TABLE document_formats (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON
  DELETE
    CASCADE REFERENCES describables(id) ON
  DELETE
    CASCADE
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
  first_editables_id BIGINT;
  second_editables_id BIGINT;
  third_editables_id BIGINT;
  fourth_editables_id BIGINT;
  fifth_editables_id BIGINT;
  sixth_editables_id BIGINT;
  seventh_editables_id BIGINT;
  eighth_editables_id BIGINT;
  ninth_editables_id BIGINT;
BEGIN
  -- We retrieve the id of the root user.
  SELECT
    id
  INTO
    root_user_id
  FROM
    users
  WHERE
    first_name = 'root'
  AND last_name = 'user';

  -- Insert the editables that indexes the formats.
  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO first_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO second_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO third_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO fourth_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO fifth_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO sixth_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO seventh_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO eighth_editables_id;

  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id) RETURNING id INTO ninth_editables_id;

  -- Insert the description of the formats in the describables table.
  INSERT INTO
    describables (id, name, description)
  VALUES
    (first_editables_id, 'pdf', 'Portable Document Format'),
    (second_editables_id, 'jpg', 'Joint Photographic Experts Group'),
    (third_editables_id, 'png', 'Portable Network Graphics'),
    (fourth_editables_id, 'mgf', 'Mascot generic format files'),
    (fifth_editables_id, 'jpeg', 'Joint Photographic Experts Group'),
    (sixth_editables_id, 'csv', 'Comma-separated values'),
    (seventh_editables_id, 'txt', 'Plain text'),
    (eighth_editables_id, 'webp', 'WebP image format'),
    (ninth_editables_id, 'json', 'JavaScript Object Notation');

  -- Insert the formats in the document_formats table.
  INSERT INTO
    document_formats (id)
  VALUES
    (first_editables_id),
    (second_editables_id),
    (third_editables_id),
    (fourth_editables_id),
    (fifth_editables_id),
    (sixth_editables_id),
    (seventh_editables_id),
    (eighth_editables_id),
    (ninth_editables_id);
END $$;
