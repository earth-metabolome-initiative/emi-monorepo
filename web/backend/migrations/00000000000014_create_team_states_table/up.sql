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
  editables_ids UUID[];
  names VARCHAR(255)[] := ARRAY['active', 'inactive', 'archived'];
  descriptions VARCHAR(255)[] := ARRAY[
    'The team is active and can be used.',
    'The team is inactive and cannot be used.',
    'The team is archived and cannot be used.'
  ];
  font_awesome_icons VARCHAR(255)[] := ARRAY['fa-check', 'fa-times', 'fa-archive'];
BEGIN
  -- Retrieve the id of the root user
  SELECT id INTO root_user_id FROM users WHERE first_name = 'root' AND last_name = 'user';

  -- Insert the editables that index the team states
  WITH inserted_rows AS (
    INSERT INTO editables (created_by)
    SELECT root_user_id FROM generate_series(1, array_length(names, 1))
    RETURNING id
  )
  SELECT array_agg(id) INTO editables_ids FROM inserted_rows;

  -- Insert the description of the team states in the describables table
  INSERT INTO describables (id, name, description)
  SELECT unnest(editables_ids), unnest(names), unnest(descriptions);

  -- Insert the team states in the team_states table
  INSERT INTO team_states (id, font_awesome_icon)
  SELECT unnest(editables_ids), unnest(font_awesome_icons);
END;
$$;