-- SQL defining the project_milestones table.
-- A project milestone is a significant event in a project,
-- such as the completion of a phase, the delivery of a product, or the start of a new phase.
-- The project_milestones table contains a project_id column to store the project that the milestone is part of,
-- a name column to store the name of the milestone, a description column to store a description of the milestone,
-- a due_date column to store the due date of the milestone,
-- and a completed_at column to store the completion date of the milestone.
-- The created_at column is used to store the creation time of the record.
-- Since only a project administrator can add a milestone to a project, the project_milestones table
-- also contains a column to specify which administrator added the milestone to the project.
CREATE TABLE project_milestones (
    id UUID PRIMARY KEY REFERENCES describables(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE
);

-- We also need to add a bi-directional cascade delete constraint to the editables
-- table, so that when a project milestone is deleted, the corresponding editable is also deleted.
-- Since the editables table is referenced by several other tables, we cannot add a cascade
-- delete constraint to the editables table. Instead, we add a trigger to delete the corresponding
-- record in the editables table when a project milestone is deleted.
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
    ON project_milestones FOR EACH ROW EXECUTE FUNCTION delete_editables();
