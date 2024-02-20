-- SQL to define the teams table.
-- The teams table is used to describe an equipe of users that work together on a project.
-- A team may be composed of one or more users, and a user may be part of one or more teams.
-- The team name is unique. A team may have a parent team, and a team may have one or more child teams.
-- The team abstraction is primarily used to manage access to projects, so to avoid having to
-- manage access to each user individually. The created_at and updated_at columns are used to store
-- the creation and last update time of the record.
CREATE TABLE teams (
  id BIGINT PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE REFERENCES describables(id) ON DELETE CASCADE,
  parent_team_id INTEGER DEFAULT NULL REFERENCES teams(id) ON DELETE CASCADE,
  team_state_id BIGINT NOT NULL REFERENCES team_states(id)
);