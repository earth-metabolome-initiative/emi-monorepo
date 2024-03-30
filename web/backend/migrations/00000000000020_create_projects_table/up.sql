-- SQL to create the projects table.
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    state_id UUID NOT NULL REFERENCES project_states(id),
    parent_project_id UUID REFERENCES projects(id) ON
    DELETE
        CASCADE,
        budget FLOAT DEFAULT NULL,
        expenses FLOAT DEFAULT NULL,
        currency VARCHAR(3) DEFAULT NULL,
        created_by UUID NOT NULL REFERENCES users(id),
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expected_end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        end_date TIMESTAMP WITH TIME ZONE DEFAULT NULL
);