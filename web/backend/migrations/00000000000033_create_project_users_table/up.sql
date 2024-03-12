-- SQL for creating the project_users table.
-- The project_users table is used to store the users that are associated with a project.
-- The user_id and project_id columns are used to store the user and project, which are used as primary keys.
-- The role column is used to store the role of the user in the project.
-- The created_at column is used to store the creation time of the record.
-- Since only a project admin can add link a user to a project, the project_users table
-- also contains a column to specify which admin added the user to the project.

CREATE TABLE project_users (
    id UUID PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES project_user_roles (id) ON DELETE CASCADE,
    UNIQUE (user_id, project_id, role_id)
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a project user is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a project user is deleted.
CREATE OR REPLACE FUNCTION delete_editables() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM
        editables
    WHERE
        id = OLD.id;

    RETURN OLD;

END;

$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_editables AFTER
DELETE
    ON project_users FOR EACH ROW EXECUTE FUNCTION delete_editables();

-- We proceed to add the 'root', the user #id 1, to the 'Earth Metabolome Initiative' project
-- as an admin. Since we do not know the id of the 'Earth Metabolome Initiative'
-- project, nor the id of the project_user_roles table curresponding to the 'admin'
-- role, we need to store them into a couple of variables first.
DO $$
DECLARE
    root_user_id UUID;
    project_id UUID;
    role_id UUID;
    editables_id UUID;
BEGIN
    -- We retrieve the id of the root user.
    SELECT
        id INTO root_user_id
    FROM
        users
    WHERE
        first_name = 'root'
        AND last_name = 'user';

    -- Since the name of the projects and the roles are NOT stored in the respective
    -- tables but in the describables table, we need to execute a JOIN query between the
    -- two respective tables and the describables table to get the id of the project and
    -- the role.
    SELECT
        projects.id
    INTO
        project_id
    FROM
        projects
        JOIN describables ON projects.id = describables.id
    WHERE
        name = 'Earth Metabolome Initiative';

    SELECT
        project_user_roles.id
    INTO
        role_id
    FROM
        project_user_roles
        JOIN describables ON project_user_roles.id = describables.id
    WHERE
        name = 'admin';

    -- We insert into the editables table the record that indexes the project user.
    INSERT INTO
        editables (created_by)
    VALUES
        (root_user_id) RETURNING id INTO editables_id;

    -- We insert the record that links the user #id 1 to the 'Earth Metabolome Initiative'
    -- project as an admin.
    INSERT INTO
        project_users (id, user_id, project_id, role_id)
    VALUES
        (editables_id, root_user_id, project_id, role_id);
END;

$$;