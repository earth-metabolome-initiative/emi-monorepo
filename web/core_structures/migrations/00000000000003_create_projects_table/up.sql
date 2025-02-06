-- SQL to create the projects table.
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    state_id INTEGER NOT NULL DEFAULT 1,
    icon_id INTEGER NOT NULL DEFAULT 415,
    color_id INTEGER NOT NULL DEFAULT 1,
    parent_project_id INTEGER,
    budget FLOAT DEFAULT NULL,
    expenses FLOAT DEFAULT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expected_end_date TIMESTAMP DEFAULT NULL,
    end_date TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (state_id) REFERENCES project_states(id),
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id),
    FOREIGN KEY (parent_project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    CONSTRAINT project_parent CHECK (id != parent_project_id)
);

CREATE FUNCTION concat_projects_name_description(
  name text,
  description text
) RETURNS text AS $$
BEGIN
  CASE
    WHEN description IS NULL THEN
      RETURN name;
    ELSE
      RETURN name || ' ' || description;
  END CASE;
END;
$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;

CREATE INDEX projects_name_description_trgm_idx ON projects USING gin (
  concat_projects_name_description(name, description) gin_trgm_ops
);
