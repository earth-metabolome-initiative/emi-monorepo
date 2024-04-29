CREATE TABLE teams (
    -- change to integer and the change it to serial in a new directoy
    id INTEGER PRIMARY KEY,
    -- a name of the team
    name TEXT NOT NULL,
    -- a description of the team
    description TEXT NOT NULL,
    parent_team_id INTEGER DEFAULT NULL REFERENCES teams(id) ON
    DELETE CASCADE
);