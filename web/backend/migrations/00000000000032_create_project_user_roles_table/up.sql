-- Your SQL goes here
CREATE TABLE project_user_roles (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE REFERENCES describables(id) ON
    DELETE
        CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a project user role is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a project user role is deleted.
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
    ON project_user_roles FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We proceed to add some standard roles to the project_user_roles table.
-- We start by inserting the editables that indixes the states.
DO $$
DECLARE
    first_editables_id BIGINT;

second_editables_id BIGINT;

third_editables_id BIGINT;

BEGIN
    -- Insert the editables that indexes the states.
    INSERT INTO
        editables (created_by)
    VALUES
        (1) RETURNING id INTO first_editables_id;

INSERT INTO
    editables (created_by)
VALUES
    (1) RETURNING id INTO second_editables_id;

INSERT INTO
    editables (created_by)
VALUES
    (1) RETURNING id INTO third_editables_id;

-- Insert the describables that describes the states.
INSERT INTO
    describables (id, name, description)
VALUES
    (
        first_editables_id,
        'admin',
        'The admin role has full access to the project.'
    ),
    (
        second_editables_id,
        'viewer',
        'The viewer role has read-only access to the project.'
    ),
    (
        third_editables_id,
        'editor',
        'The editor role has read and write access to the project.'
    );

INSERT INTO
    project_user_roles (id)
VALUES
    (first_editables_id),
    (second_editables_id),
    (third_editables_id);

END;

$$;