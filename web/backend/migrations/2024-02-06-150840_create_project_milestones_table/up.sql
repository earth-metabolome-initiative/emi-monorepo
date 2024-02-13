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
    project_id int NOT NULL,
    name varchar(255) NOT NULL,
    description text NOT NULL,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE,
    added_by int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (project_id, name),
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users (id) ON DELETE CASCADE
);
