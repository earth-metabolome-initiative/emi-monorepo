-- SQL to create the projects table.
--
-- A project is primarely characterized by its name and its description,
-- the user who created it, and then secondary metadata such as the date
-- when it was created and when it was last updated. Finally a project can
-- be currently read-only, and can be in several states such as "active",
-- "archived", "discontinued", "completed", etc. These states are available
-- in the "project_states" table, and may be extended in the future by an administator.
-- A project may also have a parent project.
-- A project has a budget, which is the amount of money allocated to the project. Subprojects
-- may have their own budget, and the sum of the budget of the subprojects should not exceed
-- the budget of the parent project. The budget is stored in the currency of the project,
-- and the currency is stored in the currency column.
-- A project may also have a predicted end date, such as when the funding for the project ends.
-- A project may also be public or private, and in the latter case, the access to the project
-- is managed by the project_editors, project_admins, and project_viewers tables.
-- The created_at and updated_at columns are used to store the creation and last update time of the record.
-- A project also has an optional URL for when the project has a separate website.
-- Finally, the project also has an optional path for the project logo.
CREATE TABLE projects (
    id BIGINT PRIMARY KEY REFERENCES editables(id) ON
    DELETE
        CASCADE REFERENCES describables(id) ON
    DELETE
        CASCADE,
        public BOOLEAN DEFAULT TRUE,
        state_id BIGINT REFERENCES project_states(id),
        parent_project_id BIGINT REFERENCES projects(id) ON
    DELETE
        CASCADE,
        budget MONEY DEFAULT NULL,
        expenses MONEY DEFAULT NULL,
        currency VARCHAR(3) DEFAULT NULL,
        expected_end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        website_url VARCHAR(255) DEFAULT NULL,
        logo_id BIGINT DEFAULT NULL REFERENCES documents(id) ON
    DELETE
    SET
        NULL
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a project is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a project is deleted.
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
    ON projects FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We create the main project, "Earth Metabolome Initiative", which is the parent of all projects.
-- First, we need to create the corresponding editable and describable records.
DO $$
DECLARE
    editables_id BIGINT;
    state_id BIGINT;
BEGIN
    INSERT INTO
        editables (created_by)
    VALUES
        (1) RETURNING id INTO editables_id;

INSERT INTO
    describables (id, name, description)
VALUES
    (
        editables_id,
        'Earth Metabolome Initiative',
        'The Earth Metabolome Initiative is a global effort to understand the metabolome of the Earth, and to make this information available to everyone.'
    );

-- We need to identify the ID of the "active" state.
-- Since the name itself is not stored in the project_states table, we need to join that
-- table with the describables table to find the ID of the "active" state.
SELECT
    project_states.id INTO state_id
FROM
    project_states
JOIN describables ON
    project_states.id = describables.id
WHERE
    name = 'active';

INSERT INTO
    projects (id, state_id)
VALUES
    (editables_id, state_id);

END;

$$;