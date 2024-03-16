-- SQL defining the team states table.
-- A team state is a state that a team may be in.
-- A team state is used to describe the state of a team, such as active, inactive, or archived.
CREATE TABLE team_states (
  id UUID PRIMARY KEY REFERENCES editables(id) ON
  DELETE
    CASCADE REFERENCES describables(id) ON
  DELETE
    CASCADE,
    font_awesome_icon VARCHAR(255) NOT NULL UNIQUE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a team state is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a team state is deleted.
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
  ON team_states FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We add some team states into the team_states table. Since it is necessary to execute
-- first the INSERT stataments into the editables and describables tables, we run a
-- transaction to ensure that the INSERT statements are executed in the correct order.
-- We retrieve as ID of the user creating the team states the root user ID.
DO $$
DECLARE
  root_user_id UUID;

editables_ids UUID [ ];

BEGIN
  -- We retrieve the id of the root user.
  SELECT
    id INTO root_user_id
  FROM
    users
  WHERE
    first_name = 'root'
    AND last_name = 'user';

-- We insert the first team state into the editables and describables tables.
WITH inserted_rows AS (
  INSERT INTO
    editables (created_by)
  VALUES
    (root_user_id),
    (root_user_id),
    (root_user_id) RETURNING id
)
SELECT
  array_agg(id) INTO editables_ids
FROM
  inserted_rows;

INSERT INTO
  describables (id, name, description)
VALUES
  (
    editables_ids [ 1 ],
    'active',
    'The team is active and can be used.'
  ),
  (
    editables_ids [ 2 ],
    'inactive',
    'The team is inactive and cannot be used.'
  ),
  (
    editables_ids [ 3 ],
    'archived',
    'The team is archived and cannot be used.'
  );

-- We insert the team states into the team_states table.
INSERT INTO
  team_states (id, font_awesome_icon)
VALUES
  (editables_ids [ 1 ], 'fa-check'),
  (editables_ids [ 2 ], 'fa-times'),
  (editables_ids [ 3 ], 'fa-archive');

END;

$$;