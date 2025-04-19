CREATE TABLE IF NOT EXISTS teams (
    -- change to integer and the change it to serial in a new directoy
    id INTEGER PRIMARY KEY,
    -- a name of the team
    name TEXT NOT NULL UNIQUE CHECK (must_not_be_empty(name)),
    -- a description of the team
    description TEXT NOT NULL,
    icon_id SMALLINT NOT NULL DEFAULT 1387,
    color_id SMALLINT NOT NULL DEFAULT 15,
    state_id SMALLINT NOT NULL DEFAULT 1,
    parent_team_id INTEGER,
    -- The user who created the team
    created_by INTEGER NOT NULL,
    -- The date the team was created
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL,
    -- The date the team was last updated
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (icon_id) REFERENCES icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id),
    FOREIGN KEY (state_id) REFERENCES team_states(id),
    FOREIGN KEY (parent_team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id),
    CONSTRAINT parent_team_circularity CHECK (must_be_distinct_i32(parent_team_id, id))
);

CREATE TABLE IF NOT EXISTS team_members (
  team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  member_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (team_id, member_id)
);

CREATE TABLE IF NOT EXISTS team_projects (
  team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  PRIMARY KEY (team_id, project_id)
);