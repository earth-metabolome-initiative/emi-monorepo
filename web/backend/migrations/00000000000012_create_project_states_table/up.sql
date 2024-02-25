-- SQL defining a state that a project may be in.
CREATE TABLE project_states (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE REFERENCES describables(id) ON
    DELETE
        CASCADE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a project state is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a project state is deleted.
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
    ON project_states FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We insert the default states for a project.
-- We start by inserting the editables that indixes the states.
DO $$
DECLARE
    root_user_id UUID;

first_editables_id BIGINT;

second_editables_id BIGINT;

third_editables_id BIGINT;

fourth_editables_id BIGINT;

BEGIN
    -- We retrieve the id of the root user.
    SELECT
        id INTO root_user_id
    FROM
        users
    WHERE
        first_name = 'root'
        AND last_name = 'user';

-- Insert the editables that indexes the states.
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

-- Insert the describables that describes the states.
INSERT INTO
    describables (id, name, description)
VALUES
    (
        first_editables_id,
        'active',
        'The project is active and can be edited.'
    ),
    (
        second_editables_id,
        'archived',
        'The project is archived and can be viewed but not edited.'
    ),
    (
        third_editables_id,
        'discontinued',
        'The project is discontinued and can be viewed but not edited.'
    ),
    (
        fourth_editables_id,
        'completed',
        'The project is completed and can be viewed but not edited.'
    );

-- Insert the project states.
INSERT INTO
    project_states (id)
VALUES
    (first_editables_id),
    (second_editables_id),
    (third_editables_id),
    (fourth_editables_id);

END $$;