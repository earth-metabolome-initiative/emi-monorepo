-- SQL to create the projects table.
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    state_id INTEGER NOT NULL REFERENCES project_states(id),
    icon_id INTEGER NOT NULL UNIQUE REFERENCES font_awesome_icons(id) ON
    DELETE
        CASCADE,
        color_id INTEGER NOT NULL UNIQUE REFERENCES colors(id) ON
    DELETE
        CASCADE,
        parent_project_id INTEGER REFERENCES projects(id) ON
    DELETE
        CASCADE,
        budget FLOAT DEFAULT NULL,
        expenses FLOAT DEFAULT NULL,
        created_by INTEGER NOT NULL REFERENCES users(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_by INTEGER NOT NULL REFERENCES users(id),
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expected_end_date TIMESTAMP DEFAULT NULL,
        end_date TIMESTAMP DEFAULT NULL
);