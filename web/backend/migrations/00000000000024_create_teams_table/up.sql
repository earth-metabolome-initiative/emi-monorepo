-- SQL to define the teams table.
-- The teams table is used to describe an equipe of users that work together on a project.
-- A team may be composed of one or more users, and a user may be part of one or more teams.
-- The team name is unique. A team may have a parent team, and a team may have one or more child teams.
-- The team abstraction is primarily used to manage access to projects, so to avoid having to
-- manage access to each user individually. The created_at and updated_at columns are used to store
-- the creation and last update time of the record.
CREATE TABLE teams (
    id UUID PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE REFERENCES describables(id) ON
    DELETE
        CASCADE,
        parent_team_id UUID DEFAULT NULL REFERENCES teams(id) ON
    DELETE
        CASCADE,
        team_state_id UUID NOT NULL REFERENCES team_states(id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a team is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a team is deleted.
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
    ON teams FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- -- We add the administrators team to the teams table. The administrators team is a special team
-- -- that has access to all aspects of the platform. The name is defined in the environment with
-- -- the ADMINISTRATORS_TEAM_NAME variable. The team is created with the team_state_id set to
-- -- the active state. We team is created with the root user as the creator.
DO $$
DECLARE
    root_user_id UUID;

team_state_id UUID;

administrators_team_id UUID;

BEGIN
    -- We retrieve the id of the root user.
    SELECT
        id INTO root_user_id
    FROM
        users
    WHERE
        first_name = 'root'
        AND last_name = 'user';

-- We retrieve the id of the active team state.
-- Since the name of the team state is stored in the describable table, we need to
-- join the team_states table with the describables table to retrieve the id of the
-- active team state.
SELECT
    team_states.id INTO team_state_id
FROM
    team_states
    JOIN describables ON team_states.id = describables.id
WHERE
    describables.name = 'active';

-- We insert the administrators team into the editables table.
INSERT INTO
    editables (created_by)
VALUES
    (root_user_id) RETURNING id INTO administrators_team_id;

-- We insert the name of the administrators team into the describables table.
INSERT INTO
    describables (id, name, description)
VALUES
    (
        administrators_team_id,
        'Administrators',
        'The administrators team has access to all aspects of the platform.'
    );

-- We insert the administrators team into the teams table.
INSERT INTO
    teams (id, team_state_id)
VALUES
    (administrators_team_id, team_state_id);

END;
$$;

-- We create a new trigger that, before a new team is being inserted into the teams table,
-- we check that the name of the team is not the same as the name of the administrators team.
-- If the name of the team is the same as the name of the administrators team, we raise an error.
CREATE
OR REPLACE FUNCTION check_administrators_team_name() RETURNS TRIGGER AS $$ BEGIN
    -- Check if the name of the new team is 'Administrators'
    IF (
        SELECT
            name
        FROM
            describables
        WHERE
            id = NEW .id
    ) = 'Administrators' THEN -- Raise an exception if the name is 'Administrators'
    RAISE
    EXCEPTION
        'The name of the team cannot be the same as the name of the administrators team.';

END IF;

-- If the condition is not met, return the new row
RETURN NEW;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER check_administrators_team_name BEFORE
INSERT
    ON teams FOR EACH ROW EXECUTE FUNCTION check_administrators_team_name();

-- Similarly, we need to make it impossible for the name of a team to be changed to
-- the name of the administrators team. We create a new trigger that, before a team
-- is being updated in the describables table, we check that the name of the team is not
-- the same as the name of the administrators team. If the name of the team is the same
-- as the name of the administrators team, we raise an error.
CREATE
OR REPLACE FUNCTION check_administrators_team_name_update() RETURNS TRIGGER AS $$ BEGIN
    -- We join the teams table with the describables table to check whether the current
    -- describables entry being updated is a team. If so, we check that the name of the
    -- team is not the same as the name of the administrators team.
    IF EXISTS (
        SELECT
            1
        FROM
            teams
            JOIN describables ON teams.id = describables.id
        WHERE
            describables.id = NEW .id
            AND describables.name = 'Administrators'
    ) THEN -- Raise an exception if the name of the team is being updated to 'Administrators'
    RAISE
    EXCEPTION
        'The name of the team cannot be changed to Administrators.';

END IF;

RETURN NEW;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER check_administrators_team_name_update BEFORE
UPDATE
    ON describables FOR EACH ROW EXECUTE FUNCTION check_administrators_team_name_update();
