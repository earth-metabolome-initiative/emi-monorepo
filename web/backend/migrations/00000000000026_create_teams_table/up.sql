CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    parent_team_id INTEGER DEFAULT NULL REFERENCES teams(id) ON
    DELETE
        CASCADE
);