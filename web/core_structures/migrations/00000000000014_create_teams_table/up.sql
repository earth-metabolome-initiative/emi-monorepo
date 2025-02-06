CREATE TABLE IF NOT EXISTS teams (
    -- change to integer and the change it to serial in a new directoy
    id INTEGER PRIMARY KEY,
    -- a name of the team
    name TEXT NOT NULL UNIQUE,
    -- a description of the team
    description TEXT NOT NULL,
    icon_id INTEGER NOT NULL DEFAULT 1387,
    color_id INTEGER NOT NULL DEFAULT 15,
    state_id INTEGER NOT NULL DEFAULT 1,
    parent_team_id INTEGER,
    -- The user who created the team
    created_by INTEGER NOT NULL,
    -- The date the team was created
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by INTEGER NOT NULL,
    -- The date the team was last updated
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (icon_id) REFERENCES font_awesome_icons(id),
    FOREIGN KEY (color_id) REFERENCES colors(id),
    FOREIGN KEY (state_id) REFERENCES team_states(id),
    FOREIGN KEY (parent_team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT parent_team_circularity CHECK (parent_team_id != id)
);

CREATE FUNCTION concat_teams_name_description(
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

CREATE INDEX teams_name_description_trgm_idx ON teams USING gin (
  concat_teams_name_description(name, description) gin_trgm_ops
);
