-- SQL to create the projects table.
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
    description TEXT NOT NULL CHECK (must_be_paragraph(description)),
    state_id SMALLINT NOT NULL DEFAULT 1,
    icon TEXT NOT NULL CHECK (must_be_font_awesome_class(icon)),
    color_id SMALLINT NOT NULL DEFAULT 1,
    parent_project_id INTEGER,
    budget REAL,
    expenses REAL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expected_end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (state_id) REFERENCES project_states(id),
    FOREIGN KEY (color_id) REFERENCES colors(id),
    FOREIGN KEY (parent_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    CHECK (must_be_distinct_i32(parent_project_id, id)),
    CHECK (must_be_distinct(name, description)),
    CHECK (must_be_smaller_than_utc(created_at, updated_at))
);
